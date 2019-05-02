//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        expr::{Owned, Slice, SliceInner},
        props::{DynMutability, Mutability, Safety},
    },
    defs::Definitions,
};

/// Implements `Borrow` or `BorrowMut`.
pub(crate) fn impl_borrow(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let ty_owned = defs.owned().outer_type();

    let trait_borrow = match mutability.into() {
        DynMutability::Constant => quote! { std::borrow::Borrow },
        DynMutability::Mutable => quote! { std::borrow::BorrowMut },
    };
    let fn_borrow = match mutability.into() {
        DynMutability::Constant => quote! { borrow },
        DynMutability::Mutable => quote! { borrow_mut },
    };

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let slice_inner_ref = {
        let owned_inner_ref = mutability.make_ref(Owned::new(quote!(self)).to_owned_inner(defs));
        let ty_owned_inner = defs.owned().inner_type();
        let ty_slice_inner = defs.slice().inner_type();
        SliceInner::new(
            quote! {
                <#ty_owned_inner as #trait_borrow<#ty_slice_inner>>::#fn_borrow(#owned_inner_ref)
            },
            mutability,
        )
    };
    let body: Slice<_, _> = slice_inner_ref.to_slice_unchecked(defs, Safety::Safe);

    let self_ref = mutability.make_ref(quote! { self });
    let ty_slice = defs.slice().outer_type();
    let ty_slice_ref = mutability.make_ref(&ty_slice);
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
    let ty_owned = defs.owned().outer_type();

    let trait_deref = match mutability.into() {
        DynMutability::Constant => quote! { std::ops::Deref },
        DynMutability::Mutable => quote! { std::ops::DerefMut },
    };
    let fn_deref = match mutability.into() {
        DynMutability::Constant => quote! { deref },
        DynMutability::Mutable => quote! { deref_mut },
    };

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let slice_inner_ref = {
        let owned_inner_ref = mutability.make_ref(Owned::new(quote!(self)).to_owned_inner(defs));
        let ty_owned_inner = defs.owned().inner_type();
        SliceInner::new(
            quote! {
                <#ty_owned_inner as #trait_deref>::#fn_deref(#owned_inner_ref)
            },
            mutability,
        )
    };
    let body: Slice<_, _> = slice_inner_ref.to_slice_unchecked(defs, Safety::Safe);

    let ty_slice = defs.slice().outer_type();
    let target = match mutability.into() {
        DynMutability::Constant => quote! { type Target = #ty_slice; },
        DynMutability::Mutable => quote! {},
    };

    let self_ref = mutability.make_ref(quote! { self });
    let ty_slice_ref = mutability.make_ref(&ty_slice);
    quote! {
        impl #trait_deref for #ty_owned {
            #target

            fn #fn_deref(#self_ref) -> #ty_slice_ref {
                #body
            }
        }
    }
}
