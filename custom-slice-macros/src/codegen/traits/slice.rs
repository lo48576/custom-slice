//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{
        expr::{Owned, Slice, SliceInner},
        props::{Constant, Mutability, Safety},
        types::{SmartPtr, SmartPtrExt},
    },
    defs::Definitions,
};

/// Implements `ToOwned`.
pub(crate) fn impl_to_owned(defs: &Definitions) -> TokenStream {
    let ty_owned = defs.ty_owned();

    // `&Slice` -> `&SliceInner` -> `OwnedInner` -> `Owned`.
    let owned: Owned<_> = Slice::new(quote!(self), Constant)
        .to_slice_inner_ref(defs)
        .to_owned_inner(defs)
        .to_owned_unchecked(defs);

    let ty_slice = defs.ty_slice();
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
    let ty_slice_ref = mutability.make_ref(defs.ty_slice());
    let ty_slice_inner_ref = mutability.make_ref(defs.ty_slice_inner());

    let default = SliceInner::new(
        quote! {
            <#ty_slice_inner_ref as std::default::Default>::default()
        },
        mutability,
    );
    let body: Slice<_, _> = default.to_slice_unchecked(defs, Safety::Safe);

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
    let ty_slice = defs.ty_slice();
    let ty_slice_inner = defs.ty_slice_inner();

    let ty_smartptr_slice = smartptr.ty(&ty_slice);
    let expr_from_raw = {
        let default_smartptr_inner = {
            let ty_smartptr_slice_inner = smartptr.ty(&ty_slice_inner);
            quote! {
                <#ty_smartptr_slice_inner as std::default::Default>::default()
            }
        };
        let expr_into_raw_inner = smartptr.expr_into_raw(ty_slice_inner, default_smartptr_inner);
        smartptr.expr_from_raw(&ty_slice, quote!(#expr_into_raw_inner as *mut #ty_slice))
    };
    quote! {
        impl std::default::Default for #ty_smartptr_slice {
            fn default() -> Self {
                unsafe { #expr_from_raw }
            }
        }
    }
}

/// Implements `From<&Slice>` for `{Arc, Box, Rc}<Slice>`.
pub(crate) fn impl_into_smartptr(defs: &Definitions, smartptr: impl SmartPtr) -> TokenStream {
    let ty_slice = defs.ty_slice();
    let ty_slice_inner = defs.ty_slice_inner();
    let arg_name = Slice::new(quote!(_v), Constant);

    let ty_smartptr_slice = smartptr.ty(&ty_slice);
    let expr_from_raw = {
        let expr_smartptr_inner = {
            let ty_smartptr_slice_inner = smartptr.ty(&ty_slice_inner);
            let arg_inner_ref: SliceInner<_, _> = arg_name.to_slice_inner_ref(defs);
            quote!(<#ty_smartptr_slice_inner>::from(#arg_inner_ref))
        };
        let expr_into_raw_inner = smartptr.expr_into_raw(ty_slice_inner, expr_smartptr_inner);
        smartptr.expr_from_raw(&ty_slice, quote!(#expr_into_raw_inner as *mut #ty_slice))
    };
    quote! {
        impl std::convert::From<&#ty_slice> for #ty_smartptr_slice {
            fn from(#arg_name: &#ty_slice) -> Self {
                unsafe { #expr_from_raw }
            }
        }
    }
}
