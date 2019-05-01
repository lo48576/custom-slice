//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        expr::{OwnedInner, SliceInner},
        props::{Mutability, Safety},
        types::{SmartPtr, SmartPtrExt},
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

    let default = SliceInner::new(
        quote! {
            <#ty_slice_inner_ref as std::default::Default>::default()
        },
        mutability,
    );
    let body = default.to_slice_unchecked(defs, Safety::Safe);

    quote! {
        impl std::default::Default for #ty_slice_ref {
            fn default() -> Self {
                #body
            }
        }
    }
}

/// Implements `Default` for `{Arc, Box, Rc}<Slice>`.
pub(crate) fn impl_default_smartptr(defs: &Definitions, smartptr: impl SmartPtr) -> TokenStream {
    let ty_slice = defs.slice().outer_type();
    let ty_slice_inner = defs.slice().inner_type();

    let ty_smartptr_slice = smartptr.ty(ty_slice);
    let expr_from_raw = {
        let default_smartptr_inner = {
            let ty_smartptr_slice_inner = smartptr.ty(ty_slice_inner);
            quote! {
                <#ty_smartptr_slice_inner as std::default::Default>::default()
            }
        };
        let expr_into_raw_inner = smartptr.expr_into_raw(ty_slice_inner, default_smartptr_inner);
        smartptr.expr_from_raw(ty_slice, quote!(#expr_into_raw_inner as *mut #ty_slice))
    };
    quote! {
        impl std::default::Default for #ty_smartptr_slice {
            fn default() -> Self {
                unsafe { #expr_from_raw }
            }
        }
    }
}
