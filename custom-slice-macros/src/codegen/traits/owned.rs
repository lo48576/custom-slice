//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        expr::SliceInner,
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
    let owned_inner_ref = {
        let owned_field = defs.owned().field_name();
        mutability.make_ref(quote! { self.#owned_field })
    };
    let slice_inner_ref = {
        let ty_owned_inner = defs.owned().inner_type();
        let ty_slice_inner = defs.slice().inner_type();
        SliceInner::new(
            quote! {
                <#ty_owned_inner as #trait_borrow<#ty_slice_inner>>::#fn_borrow(#owned_inner_ref)
            },
            mutability,
        )
    };
    let body = slice_inner_ref.to_slice_unchecked(defs, Safety::Safe);

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
