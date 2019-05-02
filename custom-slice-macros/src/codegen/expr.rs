//! Expressions.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    codegen::props::{Constant, Mutability, Safety},
    defs::Definitions,
};

/// An expression of a owned type (such as `String` for `String`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Owned<T>(T);

impl<T: ToTokens> Owned<T> {
    pub(crate) fn new(expr: T) -> Self {
        Self(expr)
    }

    pub(crate) fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<impl ToTokens> {
        defs.expr_owned_to_inner(self)
    }
}

impl<T: ToTokens> ToTokens for Owned<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// An expression of a owned inner type (such as `Vec<u8>` for `String`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct OwnedInner<T>(T);

impl<T: ToTokens> OwnedInner<T> {
    pub(crate) fn new(expr: T) -> Self {
        Self(expr)
    }

    pub(crate) fn to_slice_inner_ref(
        &self,
        defs: &Definitions,
    ) -> SliceInner<TokenStream, Constant> {
        let ty_slice_inner = defs.ty_slice_inner();
        let ty_owned_inner = defs.ty_owned_inner();
        SliceInner::new(
            quote! {
                <#ty_owned_inner as std::borrow::Borrow<#ty_slice_inner>>::borrow(&#self)
            },
            Constant,
        )
    }

    pub(crate) fn to_owned_unchecked(&self, defs: &Definitions) -> Owned<impl ToTokens> {
        defs.expr_owned_from_inner(self)
    }
}

impl<T: ToTokens> ToTokens for OwnedInner<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// An expression of a slice type (such as `&str` for `&str`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Slice<T, M> {
    expr: T,
    mutability: M,
}

impl<T: ToTokens, M: Mutability> Slice<T, M> {
    pub(crate) fn new(expr: T, mutability: M) -> Self {
        Self { expr, mutability }
    }

    pub(crate) fn mutability(&self) -> M {
        self.mutability
    }

    pub(crate) fn to_slice_inner_ref(&self, defs: &Definitions) -> SliceInner<TokenStream, M> {
        let inner = defs.expr_slice_to_inner(self);
        SliceInner::new(self.mutability.make_ref(inner), self.mutability)
    }
}

impl<T: ToTokens, M> ToTokens for Slice<T, M> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);
    }
}

/// An expression of a slice inner type (such as `&[u8]` for `&str`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct SliceInner<T, M> {
    expr: T,
    mutability: M,
}

impl<T: ToTokens, M: Mutability> SliceInner<T, M> {
    pub(crate) fn new(expr: T, mutability: M) -> Self {
        Self { expr, mutability }
    }

    pub(crate) fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        let ty_slice_inner = defs.ty_slice_inner();
        OwnedInner(quote! {
            <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&#self)
        })
    }

    pub(crate) fn to_slice_unchecked(
        &self,
        defs: &Definitions,
        context: Safety,
    ) -> Slice<TokenStream, M> {
        let ty_slice_inner_ptr = self.mutability.make_ptr(defs.ty_slice_inner());
        let ty_slice_ptr = self.mutability.make_ptr(defs.ty_slice());
        // Type: `&#ty_slice` or `&mut #ty_slice`.
        let base = self.mutability.make_ref(quote! {
            *(#self as #ty_slice_inner_ptr as #ty_slice_ptr)
        });
        Slice::new(context.wrap_unsafe_expr(base), self.mutability)
    }
}

impl<T: ToTokens, M> ToTokens for SliceInner<T, M> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);
    }
}
