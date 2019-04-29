//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        expr::{OwnedInner, SliceInner},
        props::{Mutability, Safety},
        traits::slice_inner_to_outer_unchecked,
    },
    defs::Definitions,
};

/// Implements `ToOwned`.
pub(crate) fn impl_to_owned(defs: &Definitions) -> TokenStream {
    let ty_owned = defs.owned().outer_type();

    // `&Slice` -> `&SliceInner` -> `OwnedInner` -> `Owned`.
    let slice_inner = defs.slice().inner_expr(quote! { self });
    let owned_inner = {
        let ty_slice_inner = defs.slice().inner_type();
        OwnedInner::new(quote! {
            <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&#slice_inner)
        })
    };
    let owned = owned_inner.to_owned_unchecked(defs);

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

/// Implements `Default` for `&Slice` or `&mut Slice`.
pub(crate) fn impl_default_ref(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let ty_slice_ref = mutability.make_ref(defs.slice().outer_type());
    let ty_slice_inner_ref = mutability.make_ref(defs.slice().inner_type());

    let default = SliceInner::new(quote! {
        <#ty_slice_inner_ref as std::default::Default>::default()
    });
    let body = slice_inner_to_outer_unchecked(defs, default, Safety::Safe, mutability);

    quote! {
        impl std::default::Default for #ty_slice_ref {
            fn default() -> Self {
                #body
            }
        }
    }
}
