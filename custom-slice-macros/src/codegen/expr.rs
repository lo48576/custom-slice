//! Expressions.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::defs::Definitions;

pub(crate) trait SliceLikeExpression {
    /// Returns expression of type `&SliceInner`.
    fn to_slice_inner_ref(&self, defs: &Definitions) -> SliceInner<TokenStream>;
    /// Returns expression of type `OwnedInner`.
    fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream>;
}

/// An expression of a owned type (such as `String` for `String`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Owned<T>(pub T);

impl<T> Owned<T> {
    #[allow(dead_code)]
    pub(crate) fn as_ref(&self) -> Owned<&T> {
        Owned(&self.0)
    }
}

impl<T: ToTokens> ToTokens for Owned<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: ToTokens> SliceLikeExpression for Owned<T> {
    fn to_slice_inner_ref(&self, defs: &Definitions) -> SliceInner<TokenStream> {
        self.to_owned_inner(defs).to_slice_inner_ref(defs)
    }

    fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        OwnedInner(defs.owned().inner_expr(self))
    }
}

/// An expression of a owned inner type (such as `Vec<u8>` for `String`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct OwnedInner<T>(pub T);

impl<T> OwnedInner<T> {
    pub(crate) fn as_ref(&self) -> OwnedInner<&T> {
        OwnedInner(&self.0)
    }
}

impl<T: ToTokens> ToTokens for OwnedInner<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: ToTokens> SliceLikeExpression for OwnedInner<T> {
    fn to_slice_inner_ref(&self, defs: &Definitions) -> SliceInner<TokenStream> {
        let ty_slice_inner = defs.slice().inner_type();
        let ty_owned_inner = defs.owned().inner_type();
        SliceInner(quote! {
            <#ty_owned_inner as std::borrow::Borrow<#ty_slice_inner>>::borrow(&#self)
        })
    }

    fn to_owned_inner(&self, _: &Definitions) -> OwnedInner<TokenStream> {
        OwnedInner(self.into_token_stream())
    }
}

/// An expression of a slice type (such as `&str` for `&str`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Slice<T>(pub T);

impl<T> Slice<T> {
    #[allow(dead_code)]
    pub(crate) fn as_ref(&self) -> Slice<&T> {
        Slice(&self.0)
    }
}

impl<T: ToTokens> ToTokens for Slice<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: ToTokens> SliceLikeExpression for Slice<T> {
    fn to_slice_inner_ref(&self, defs: &Definitions) -> SliceInner<TokenStream> {
        let inner = defs.slice().inner_expr(self);
        SliceInner(quote! { &#inner })
    }

    fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        self.to_slice_inner_ref(defs).to_owned_inner(defs)
    }
}

/// An expression of a slice inner type (such as `&[u8]` for `&str`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct SliceInner<T>(pub T);

impl<T> SliceInner<T> {
    pub(crate) fn as_ref(&self) -> SliceInner<&T> {
        SliceInner(&self.0)
    }
}

impl<T: ToTokens> ToTokens for SliceInner<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: ToTokens> SliceLikeExpression for SliceInner<T> {
    fn to_slice_inner_ref(&self, _: &Definitions) -> SliceInner<TokenStream> {
        SliceInner(self.into_token_stream())
    }

    fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        let ty_slice_inner = defs.slice().inner_type();
        OwnedInner(quote! {
            <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&#self)
        })
    }
}
