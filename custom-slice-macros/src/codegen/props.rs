//! Properties.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) trait Mutability: Sized + Copy + Into<DynMutability> {
    fn make_ref(self, following: impl ToTokens) -> TokenStream;
    fn make_ptr(self, following: impl ToTokens) -> TokenStream;
}

/// Mutability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynMutability {
    /// Mutable.
    Mutable,
    /// Constant.
    Constant,
}

impl Mutability for DynMutability {
    fn make_ref(self, following: impl ToTokens) -> TokenStream {
        match self {
            DynMutability::Mutable => Mutable.make_ref(following),
            DynMutability::Constant => Constant.make_ref(following),
        }
    }

    fn make_ptr(self, following: impl ToTokens) -> TokenStream {
        match self {
            DynMutability::Mutable => Mutable.make_ptr(following),
            DynMutability::Constant => Constant.make_ptr(following),
        }
    }
}

impl From<Mutable> for DynMutability {
    fn from(_: Mutable) -> Self {
        DynMutability::Mutable
    }
}

impl From<Constant> for DynMutability {
    fn from(_: Constant) -> Self {
        DynMutability::Constant
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Mutable;

impl Mutability for Mutable {
    fn make_ref(self, following: impl ToTokens) -> TokenStream {
        quote! { &mut #following }
    }

    fn make_ptr(self, following: impl ToTokens) -> TokenStream {
        quote! { *mut #following }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Constant;

impl Mutability for Constant {
    fn make_ref(self, following: impl ToTokens) -> TokenStream {
        quote! { &#following }
    }

    fn make_ptr(self, following: impl ToTokens) -> TokenStream {
        quote! { *const #following }
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
