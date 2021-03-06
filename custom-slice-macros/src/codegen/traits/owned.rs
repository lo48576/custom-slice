//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    codegen::{
        expr::{Owned, OwnedInner, Slice, SliceInner},
        props::{Constant, DynMutability, Mutability, Safety},
        traits::{CmpTrait, OwnedToSliceTrait},
        types::RefType,
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

/// Implements `PartialEq` and `PartialOrd` using comparison of `Slice` type.
pub(crate) fn impl_cmp(defs: &Definitions, target: CmpTrait) -> TokenStream {
    target
        .impl_with_slice(defs, RefType::Owned, RefType::Owned)
        .into_token_stream()
}

/// Implements `PartialEq` and `PartialOrd` for many types.
pub(crate) fn impl_cmp_bulk(defs: &Definitions, target: CmpTrait) -> TokenStream {
    let mut tokens = TokenStream::new();
    target
        .impl_with_slice(defs, RefType::Owned, RefType::Slice)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::Slice, RefType::Owned)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::Owned, RefType::RefSlice)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::RefSlice, RefType::Owned)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::Owned, RefType::CowSlice)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::CowSlice, RefType::Owned)
        .to_tokens(&mut tokens);

    tokens
}

/// Implements `PartialEq` and `PartialOrd` for many types.
pub(crate) fn impl_cmp_inner_bulk(defs: &Definitions, target: CmpTrait) -> TokenStream {
    let mut tokens = TokenStream::new();
    target
        .impl_with_inner(defs, RefType::Owned, RefType::SliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::SliceInner, RefType::Owned)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::Owned, RefType::RefSliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::RefSliceInner, RefType::Owned)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::Owned, RefType::CowSliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::CowSliceInner, RefType::Owned)
        .to_tokens(&mut tokens);

    target
        .impl_with_inner(defs, RefType::Owned, RefType::OwnedInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::OwnedInner, RefType::Owned)
        .to_tokens(&mut tokens);

    tokens
}

/// Implements `Deref` or `DerefMut`.
pub(crate) fn impl_deref(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let trait_deref = OwnedToSliceTrait::Deref.trait_path(mutability);
    let fn_deref = OwnedToSliceTrait::Deref.method_name(mutability);

    let ty_owned = defs.ty_owned();
    let self_ref = mutability.make_ref(quote!(self));
    let target = match mutability.into() {
        DynMutability::Constant => {
            let ty_slice = defs.ty_slice();
            quote!(type Target = #ty_slice;)
        }
        DynMutability::Mutable => quote!(),
    };
    let ty_ret = mutability.make_ref(quote!(Self::Target));

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let body: Slice<_, _> = Owned::new(quote!(self))
        .to_owned_inner(defs)
        .to_slice_inner_ref(defs, OwnedToSliceTrait::Deref, mutability)
        .to_slice_unchecked(defs, Safety::Safe);
    quote! {
        impl #trait_deref for #ty_owned {
            #target

            fn #fn_deref(#self_ref) -> #ty_ret {
                #body
            }
        }
    }
}

/// Implements `From<OwnedInner>`.
pub(crate) fn impl_from_inner(defs: &Definitions) -> TokenStream {
    if defs.has_validator() {
        panic!("`From<OwnedInner>` cannot be implemented because a validator is specified");
    }

    let arg_name = OwnedInner::new(quote!(_v));
    let ty_owned = defs.ty_owned();
    let ty_owned_inner = defs.ty_owned_inner();
    let body: Owned<_> = arg_name.to_owned_unchecked(defs);
    quote! {
        impl std::convert::From<#ty_owned_inner> for #ty_owned {
            fn from(#arg_name: #ty_owned_inner) -> Self {
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

/// Implements `TryFrom<OwnedInner>`.
pub(crate) fn impl_try_from_inner(defs: &Definitions) -> TokenStream {
    let arg_name = OwnedInner::new(quote!(_v));
    let error_var = &quote!(_e);

    let ty_owned = defs.ty_owned();
    let ty_owned_inner = defs.ty_owned_inner();

    let (body, ty_error) = inner_to_outer_checked(defs, arg_name.as_ref(), error_var);
    quote! {
        impl std::convert::TryFrom<#ty_owned_inner> for #ty_owned {
            type Error = #ty_error;

            fn try_from(#arg_name: #ty_owned_inner) -> std::result::Result<Self, Self::Error> {
                #body
            }
        }
    }
}

/// Returns `(expr_result_outer, ty_error)`.
pub(crate) fn inner_to_outer_checked(
    defs: &Definitions,
    inner_var: OwnedInner<impl ToTokens>,
    error_var: impl ToTokens,
) -> (TokenStream, syn::Type) {
    let (ty_error, mapped_error) = defs.owned_error_ty_and_val(&error_var, inner_var.as_ref());

    let expr_owned = inner_var.to_owned_unchecked(defs);
    let expr_slice_inner_ref =
        inner_var.to_slice_inner_ref(defs, OwnedToSliceTrait::Borrow, Constant);
    let fn_validate = defs.fn_validator().unwrap_or_else(|| {
        panic!("Validator should be necessary for `TryFromInner` derive target")
    });
    let expr = quote! {
        match #fn_validate(#expr_slice_inner_ref) {
            Ok(_) => Ok(#expr_owned),
            Err(#error_var) => Err(#mapped_error),
        }
    };
    (expr, ty_error)
}
