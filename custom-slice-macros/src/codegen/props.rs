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
