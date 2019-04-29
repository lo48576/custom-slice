//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        props::{Mutability, Safety},
        traits::slice_inner_to_outer_unchecked,
    },
    defs::Definitions,
};

/// Implements `Borrow` or `BorrowMut`.
pub(crate) fn impl_borrow(defs: &Definitions, mutability: Mutability) -> TokenStream {
    let ty_owned = defs.owned().outer_type();

    let trait_borrow = match mutability {
        Mutability::Constant => quote! { std::borrow::Borrow },
        Mutability::Mutable => quote! { std::borrow::BorrowMut },
    };
    let fn_borrow = match mutability {
        Mutability::Constant => quote! { borrow },
        Mutability::Mutable => quote! { borrow_mut },
    };

    // `&Owned` -> `&OwnedInner` -> `&SliceInner` -> `&Slice`.
    let owned_inner_ref = {
        let owned_field = defs.owned().field_name();
        mutability.make_ref(quote! { self.#owned_field })
    };
    let slice_inner_ref = {
        let ty_owned_inner = defs.owned().inner_type();
        let ty_slice_inner = defs.slice().inner_type();
        quote! {
            <#ty_owned_inner as #trait_borrow<#ty_slice_inner>>::#fn_borrow(#owned_inner_ref)
        }
    };
    let body = slice_inner_to_outer_unchecked(defs, slice_inner_ref, Safety::Safe, mutability);

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
