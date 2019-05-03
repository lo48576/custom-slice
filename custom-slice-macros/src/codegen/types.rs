//! Types-related stuff.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    codegen::{
        expr::{Owned, Slice},
        props::Constant,
        traits::OwnedToSliceTrait,
    },
    defs::Definitions,
};

pub(crate) trait SmartPtr {
    fn ty(&self, ty_inner: impl ToTokens) -> TokenStream;
    fn method_from_raw(&self) -> TokenStream;
    fn method_into_raw(&self) -> TokenStream;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StdSmartPtr {
    /// `std::sync::Arc`.
    Arc,
    /// `std::boxed::Box`.
    Box,
    /// `std::rc::Rc`.
    Rc,
}

impl SmartPtr for StdSmartPtr {
    fn ty(&self, ty_inner: impl ToTokens) -> TokenStream {
        match self {
            StdSmartPtr::Arc => quote!(std::sync::Arc<#ty_inner>),
            StdSmartPtr::Box => quote!(std::boxed::Box<#ty_inner>),
            StdSmartPtr::Rc => quote!(std::rc::Rc<#ty_inner>),
        }
    }

    fn method_from_raw(&self) -> TokenStream {
        quote!(from_raw)
    }

    fn method_into_raw(&self) -> TokenStream {
        quote!(into_raw)
    }
}

pub(crate) trait SmartPtrExt: SmartPtr {
    fn expr_from_raw(&self, ty_inner: impl ToTokens, expr: impl ToTokens) -> TokenStream {
        let ty = self.ty(ty_inner);
        let method = self.method_from_raw();
        quote!(<#ty>::#method(#expr))
    }

    fn expr_into_raw(&self, ty_inner: impl ToTokens, expr: impl ToTokens) -> TokenStream {
        let ty = self.ty(ty_inner);
        let method = self.method_into_raw();
        quote!(<#ty>::#method(#expr))
    }
}

impl<T: SmartPtr> SmartPtrExt for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RefType {
    /// `Slice`.
    Slice,
    /// `&Slice`.
    RefSlice,
    /// `Owned`.
    Owned,
    /// `Cow<Slice>`.
    CowSlice,
    /// `SliceInner`.
    SliceInner,
    /// `&SliceInner`.
    RefSliceInner,
    /// `OwnedInner`.
    OwnedInner,
    /// `Cow<SliceInner>`.
    CowSliceInner,
}

impl RefType {
    pub(crate) fn ty(self, defs: &Definitions) -> TokenStream {
        match self {
            RefType::Slice => defs.ty_slice().into_token_stream(),
            RefType::RefSlice => {
                let ty_slice = defs.ty_slice();
                quote!(&#ty_slice)
            }
            RefType::Owned => defs.ty_owned().into_token_stream(),
            RefType::CowSlice => {
                let ty_slice = defs.ty_slice();
                quote!(std::borrow::Cow<'_, #ty_slice>)
            }
            RefType::SliceInner => defs.ty_slice_inner().into_token_stream(),
            RefType::RefSliceInner => {
                let ty_slice_inner = defs.ty_slice_inner();
                quote!(&#ty_slice_inner)
            }
            RefType::OwnedInner => defs.ty_owned_inner().into_token_stream(),
            RefType::CowSliceInner => {
                let ty_slice_inner = defs.ty_slice_inner();
                quote!(std::borrow::Cow<'_, #ty_slice_inner>)
            }
        }
    }

    /// Converts the given reference type expression to `&Slice` type.
    pub(crate) fn ref_to_slice_ref(self, defs: &Definitions, expr: impl ToTokens) -> TokenStream {
        let ty_slice = defs.ty_slice();
        match self {
            RefType::Slice => expr.into_token_stream(),
            RefType::RefSlice => quote!(*#expr),
            RefType::Owned | RefType::CowSlice => {
                quote!(std::borrow::Borrow::<#ty_slice>::borrow(#expr))
            }
            RefType::SliceInner
            | RefType::RefSliceInner
            | RefType::OwnedInner
            | RefType::CowSliceInner => unreachable!(
                "Should never happen: {:?} is not directly convertible to slice reference type",
                self
            ),
        }
    }

    /// Converts the given reference type expression to `&SliceInner` type.
    pub(crate) fn ref_to_slice_inner_ref(
        self,
        defs: &Definitions,
        expr: impl ToTokens,
    ) -> TokenStream {
        let ty_slice = defs.ty_slice();
        let ty_slice_inner = defs.ty_slice_inner();
        match self {
            RefType::Slice => Slice::new(expr, Constant)
                .to_slice_inner_ref(defs)
                .into_token_stream(),
            RefType::RefSlice => Slice::new(quote!(*#expr), Constant)
                .to_slice_inner_ref(defs)
                .into_token_stream(),
            RefType::Owned => Owned::new(expr)
                .to_owned_inner(defs)
                .to_slice_inner_ref(defs, OwnedToSliceTrait::Borrow, Constant)
                .into_token_stream(),
            RefType::CowSlice => Slice::new(
                quote!(std::borrow::Borrow::<#ty_slice>::borrow(#expr)),
                Constant,
            )
            .to_slice_inner_ref(defs)
            .into_token_stream(),
            RefType::SliceInner => expr.into_token_stream(),
            RefType::RefSliceInner => quote!(*#expr),
            RefType::OwnedInner | RefType::CowSliceInner => {
                quote!(std::borrow::Borrow::<#ty_slice_inner>::borrow(#expr))
            }
        }
    }
}
