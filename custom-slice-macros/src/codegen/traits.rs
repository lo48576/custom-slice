//! Trait impls.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    codegen::{
        expr::{Owned, OwnedInner},
        props::{Mutability, Safety},
    },
    defs::Definitions,
};

pub(crate) mod owned;
pub(crate) mod slice;

/// Returns the expression converted to a slice type without validation.
// `expr_inner_ref`: `&SliceInner` or `&mut SliceInner`.
fn slice_inner_to_outer_unchecked(
    defs: &Definitions,
    expr_inner_ref: impl ToTokens,
    context: Safety,
    mutability: Mutability,
) -> TokenStream {
    let ty_slice_inner_ptr = mutability.make_ptr(defs.slice().inner_type());
    let ty_slice_ptr = mutability.make_ptr(defs.slice().outer_type());
    // Type: `&#ty_slice` or `&mut #ty_slice`.
    let base = mutability.make_ref(quote! {
        *(#expr_inner_ref as #ty_slice_inner_ptr as #ty_slice_ptr)
    });
    context.wrap_unsafe_expr(base)
}

/// Returns the expression converted to an owned type without validation.
fn owned_inner_to_outer_unchecked(
    defs: &Definitions,
    owned_inner: OwnedInner<impl ToTokens>,
) -> Owned<TokenStream> {
    let ty_owned = defs.owned().outer_type();
    let field_owned = defs.owned().field_name();
    // Type: #ty_owned
    Owned(quote! {
        #ty_owned { #field_owned: #owned_inner }
    })
}
