//! Types-related stuff.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

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
