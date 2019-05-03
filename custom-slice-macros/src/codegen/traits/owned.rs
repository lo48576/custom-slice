//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        expr::{Owned, OwnedInner, Slice, SliceInner},
        props::{DynMutability, Mutability, Safety},
        traits::OwnedToSliceTrait,
    },
    defs::Definitions,
};

/// Implements `AsRef<Slice>` or `AsMut<Slice>`.
pub(crate) fn impl_as_ref_slice(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let trait_as_ref = OwnedToSliceTrait::AsRef.trait_path(mutability);
    let fn_as_ref = OwnedToSliceTrait::AsRef.method_name(mutability);

    let ty_owned = defs.ty_owned();
    let ty_slice = defs.ty_slice();
    let ty_slice_ref = mutability.make_ref(&ty_slice);
    let self_ref = mutability.make_ref(quote!(self));

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let body: Slice<_, _> = Owned::new(quote!(self))
        .to_owned_inner(defs)
        .to_slice_inner_ref(defs, OwnedToSliceTrait::AsRef, mutability)
        .to_slice_unchecked(defs, Safety::Safe);
    quote! {
        impl #trait_as_ref<#ty_slice> for #ty_owned {
            fn #fn_as_ref(#self_ref) -> #ty_slice_ref {
                #body
            }
        }
    }
}

/// Implements `AsRef<SliceInner>` or `AsMut<SliceInner>`.
pub(crate) fn impl_as_ref_slice_inner(
    defs: &Definitions,
    mutability: impl Mutability,
) -> TokenStream {
    let trait_as_ref = OwnedToSliceTrait::AsRef.trait_path(mutability);
    let fn_as_ref = OwnedToSliceTrait::AsRef.method_name(mutability);

    let ty_owned = defs.ty_owned();
    let ty_slice_inner = defs.ty_slice_inner();
    let ty_slice_inner_ref = mutability.make_ref(&ty_slice_inner);
    let self_ref = mutability.make_ref(quote!(self));

    // `&Owned` -> `&OwnedInner` -> `&SliceInner`.
    let body: SliceInner<_, _> = Owned::new(quote!(self))
        .to_owned_inner(defs)
        .to_slice_inner_ref(defs, OwnedToSliceTrait::AsRef, mutability);
    quote! {
        impl #trait_as_ref<#ty_slice_inner> for #ty_owned {
            fn #fn_as_ref(#self_ref) -> #ty_slice_inner_ref {
                #body
            }
        }
    }
}

/// Implements `Borrow` or `BorrowMut`.
pub(crate) fn impl_borrow(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let trait_borrow = OwnedToSliceTrait::Borrow.trait_path(mutability);
    let fn_borrow = OwnedToSliceTrait::Borrow.method_name(mutability);

    let ty_owned = defs.ty_owned();
    let ty_slice = defs.ty_slice();
    let ty_slice_ref = mutability.make_ref(&ty_slice);
    let self_ref = mutability.make_ref(quote!(self));

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let body: Slice<_, _> = Owned::new(quote!(self))
        .to_owned_inner(defs)
        .to_slice_inner_ref(defs, OwnedToSliceTrait::Borrow, mutability)
        .to_slice_unchecked(defs, Safety::Safe);
    quote! {
        impl #trait_borrow<#ty_slice> for #ty_owned {
            fn #fn_borrow(#self_ref) -> #ty_slice_ref {
                #body
            }
        }
    }
}

/// Implements `Deref` or `DerefMut`.
pub(crate) fn impl_deref(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let trait_deref = OwnedToSliceTrait::Deref.trait_path(mutability);
    let fn_deref = OwnedToSliceTrait::Deref.method_name(mutability);

    let ty_owned = defs.ty_owned();
    let ty_slice = defs.ty_slice();
    let ty_slice_ref = mutability.make_ref(&ty_slice);
    let self_ref = mutability.make_ref(quote!(self));
    let target = match mutability.into() {
        DynMutability::Constant => quote!(type Target = #ty_slice;),
        DynMutability::Mutable => quote!(),
    };

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let body: Slice<_, _> = Owned::new(quote!(self))
        .to_owned_inner(defs)
        .to_slice_inner_ref(defs, OwnedToSliceTrait::Deref, mutability)
        .to_slice_unchecked(defs, Safety::Safe);
    quote! {
        impl #trait_deref for #ty_owned {
            #target

            fn #fn_deref(#self_ref) -> #ty_slice_ref {
                #body
            }
        }
    }
}

/// Implements `Into<OwnedInner>` (actually `From<Owned> for OwnedInner`).
pub(crate) fn impl_into_inner(defs: &Definitions) -> TokenStream {
    let ty_owned = defs.ty_owned();
    let ty_owned_inner = defs.ty_owned_inner();
    let arg_name = Owned::new(quote!(_v));
    let body: OwnedInner<_> = arg_name.to_owned_inner(defs);
    quote! {
        impl std::convert::From<#ty_owned> for #ty_owned_inner {
            fn from(#arg_name: #ty_owned) -> Self {
                #body
            }
        }
    }
}
