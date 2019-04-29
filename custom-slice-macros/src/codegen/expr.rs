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
    #[allow(dead_code)]
    pub(crate) fn new(expr: T) -> Self {
        Self(expr)
    }

    #[allow(dead_code)]
    pub(crate) fn as_ref(&self) -> Owned<&T> {
        Owned(&self.0)
    }

    #[allow(dead_code)]
    pub(crate) fn to_slice_inner_ref(
        &self,
        defs: &Definitions,
    ) -> SliceInner<TokenStream, Constant> {
        self.to_owned_inner(defs).to_slice_inner_ref(defs)
    }

    #[allow(dead_code)]
    pub(crate) fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        OwnedInner(defs.owned().inner_expr(self))
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

    #[allow(dead_code)]
    pub(crate) fn as_ref(&self) -> OwnedInner<&T> {
        OwnedInner(&self.0)
    }

    pub(crate) fn to_slice_inner_ref(
        &self,
        defs: &Definitions,
    ) -> SliceInner<TokenStream, Constant> {
        let ty_slice_inner = defs.slice().inner_type();
        let ty_owned_inner = defs.owned().inner_type();
        SliceInner::new(
            quote! {
                <#ty_owned_inner as std::borrow::Borrow<#ty_slice_inner>>::borrow(&#self)
            },
            Constant,
        )
    }

    pub(crate) fn to_owned_unchecked(&self, defs: &Definitions) -> Owned<TokenStream> {
        let ty_owned = defs.owned().outer_type();
        let field_owned = defs.owned().field_name();
        Owned(quote! {
            #ty_owned { #field_owned: #self }
        })
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

    #[allow(dead_code)]
    pub(crate) fn as_ref(&self) -> Slice<&T, M> {
        Slice::new(&self.expr, self.mutability)
    }

    #[allow(dead_code)]
    pub(crate) fn to_slice_inner_ref(&self, defs: &Definitions) -> SliceInner<TokenStream, M> {
        let inner = defs.slice().inner_expr(self);
        SliceInner::new(self.mutability.make_ref(inner), self.mutability)
    }

    #[allow(dead_code)]
    pub(crate) fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        self.to_slice_inner_ref(defs).to_owned_inner(defs)
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

    #[allow(dead_code)]
    pub(crate) fn as_ref(&self) -> SliceInner<&T, M> {
        SliceInner::new(&self.expr, self.mutability)
    }

    pub(crate) fn to_owned_inner(&self, defs: &Definitions) -> OwnedInner<TokenStream> {
        let ty_slice_inner = defs.slice().inner_type();
        OwnedInner(quote! {
            <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&#self)
        })
    }

    pub(crate) fn to_slice_unchecked(
        &self,
        defs: &Definitions,
        context: Safety,
    ) -> Slice<TokenStream, M> {
        let ty_slice_inner_ptr = self.mutability.make_ptr(defs.slice().inner_type());
        let ty_slice_ptr = self.mutability.make_ptr(defs.slice().outer_type());
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
