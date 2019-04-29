//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{expr::OwnedInner, traits::owned_inner_to_outer_unchecked},
    defs::Definitions,
};

/// Implements `ToOwned`.
pub(crate) fn impl_to_owned(defs: &Definitions) -> TokenStream {
    let ty_owned = defs.owned().outer_type();

    // `&Slice` -> `&SliceInner` -> `OwnedInner` -> `Owned`.
    let slice_inner = defs.slice().inner_expr(quote! { self });
    let owned_inner = {
        let ty_slice_inner = defs.slice().inner_type();
        OwnedInner(quote! { <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&#slice_inner) })
    };
    let owned = owned_inner_to_outer_unchecked(defs, owned_inner);

    let ty_slice = defs.slice().outer_type();
    quote! {
        impl std::borrow::ToOwned for #ty_slice {
            type Owned = #ty_owned;

            fn to_owned(&self) -> Self::Owned {
                #owned
            }
        }
    }
}
