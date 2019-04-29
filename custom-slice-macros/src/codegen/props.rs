//! Properties.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Mutability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mutability {
    /// Mutable.
    Mutable,
    /// Constant.
    Constant,
}

impl Mutability {
    pub(crate) fn make_ref(self, following: impl ToTokens) -> TokenStream {
        match self {
            Mutability::Mutable => quote! { &mut #following },
            Mutability::Constant => quote! { &#following },
        }
    }

    pub(crate) fn make_ptr(self, following: impl ToTokens) -> TokenStream {
        match self {
            Mutability::Mutable => quote! { *mut #following },
            Mutability::Constant => quote! { *const #following },
        }
    }
}

/// Safety (and unsafety).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Safety {
    /// Safe.
    Safe,
    /// Unsafe.
    Unsafe,
}

impl Safety {
    #[allow(dead_code)]
    pub(crate) fn is_safe(self) -> bool {
        self == Safety::Safe
    }

    #[allow(dead_code)]
    pub(crate) fn is_unsafe(self) -> bool {
        self == Safety::Unsafe
    }

    /// Wraps the given expression with `unsafe {}` if necessary.
    ///
    /// `self` is safety of the current context.
    pub(crate) fn wrap_unsafe_expr(self, unsafe_expr: impl ToTokens) -> TokenStream {
        match self {
            Safety::Safe => quote! { unsafe { #unsafe_expr } },
            Safety::Unsafe => unsafe_expr.into_token_stream(),
        }
    }
}

impl From<Option<syn::token::Unsafe>> for Safety {
    fn from(unsafety: Option<syn::token::Unsafe>) -> Self {
        if unsafety.is_some() {
            Safety::Unsafe
        } else {
            Safety::Safe
        }
    }
}

impl From<Option<&syn::token::Unsafe>> for Safety {
    fn from(unsafety: Option<&syn::token::Unsafe>) -> Self {
        if unsafety.is_some() {
            Safety::Unsafe
        } else {
            Safety::Safe
        }
    }
}

impl From<&Option<syn::token::Unsafe>> for Safety {
    fn from(unsafety: &Option<syn::token::Unsafe>) -> Self {
        if unsafety.is_some() {
            Safety::Unsafe
        } else {
            Safety::Safe
        }
    }
}
