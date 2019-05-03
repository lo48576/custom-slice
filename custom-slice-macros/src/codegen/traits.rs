//! Trait impls.

use quote::{quote, ToTokens};

use crate::codegen::props::{DynMutability, Mutability};

pub(crate) mod owned;
pub(crate) mod slice;

/// Traits to convert from owned type to slice type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum OwnedToSliceTrait {
    /// `AsRef` and `AsRefMut`.
    AsRef,
    /// `Borrow` and `BorrowMut`.
    Borrow,
    /// `Deref` and `DerefMut`.
    Deref,
}

impl OwnedToSliceTrait {
    fn trait_path(self, mutability: impl Mutability) -> impl ToTokens {
        match (self, mutability.into()) {
            (OwnedToSliceTrait::AsRef, DynMutability::Constant) => quote!(std::convert::AsRef),
            (OwnedToSliceTrait::AsRef, DynMutability::Mutable) => quote!(std::convert::AsMut),
            (OwnedToSliceTrait::Borrow, DynMutability::Constant) => quote!(std::borrow::Borrow),
            (OwnedToSliceTrait::Borrow, DynMutability::Mutable) => quote!(std::borrow::BorrowMut),
            (OwnedToSliceTrait::Deref, DynMutability::Constant) => quote!(std::ops::Deref),
            (OwnedToSliceTrait::Deref, DynMutability::Mutable) => quote!(std::ops::DerefMut),
        }
    }
    fn trait_path_with_param(
        self,
        ty_slice: impl ToTokens,
        mutability: impl Mutability,
    ) -> impl ToTokens {
        match (self, mutability.into()) {
            (OwnedToSliceTrait::AsRef, DynMutability::Constant) => {
                quote!(std::convert::AsRef<#ty_slice>)
            }
            (OwnedToSliceTrait::AsRef, DynMutability::Mutable) => {
                quote!(std::convert::AsMut<#ty_slice>)
            }
            (OwnedToSliceTrait::Borrow, DynMutability::Constant) => {
                quote!(std::borrow::Borrow<#ty_slice>)
            }
            (OwnedToSliceTrait::Borrow, DynMutability::Mutable) => {
                quote!(std::borrow::BorrowMut<#ty_slice>)
            }
            (OwnedToSliceTrait::Deref, DynMutability::Constant) => quote!(std::ops::Deref),
            (OwnedToSliceTrait::Deref, DynMutability::Mutable) => quote!(std::ops::DerefMut),
        }
    }

    fn method_name(self, mutability: impl Mutability) -> impl ToTokens {
        match (self, mutability.into()) {
            (OwnedToSliceTrait::AsRef, DynMutability::Constant) => quote!(as_ref),
            (OwnedToSliceTrait::AsRef, DynMutability::Mutable) => quote!(as_mut),
            (OwnedToSliceTrait::Borrow, DynMutability::Constant) => quote!(borrow),
            (OwnedToSliceTrait::Borrow, DynMutability::Mutable) => quote!(borrow_mut),
            (OwnedToSliceTrait::Deref, DynMutability::Constant) => quote!(deref),
            (OwnedToSliceTrait::Deref, DynMutability::Mutable) => quote!(deref_mut),
        }
    }

    fn path_method(
        self,
        ty_owned: impl ToTokens,
        ty_slice: impl ToTokens,
        mutability: impl Mutability,
    ) -> impl ToTokens {
        let trait_ = self.trait_path_with_param(ty_slice, mutability);
        let method = self.method_name(mutability);
        quote!(<#ty_owned as #trait_>::#method)
    }

    pub(crate) fn expr_call(
        self,
        ty_owned: impl ToTokens,
        ty_slice: impl ToTokens,
        expr_owned_ref: impl ToTokens,
        mutability: impl Mutability,
    ) -> impl ToTokens {
        let path_method = self.path_method(ty_owned, ty_slice, mutability);
        quote!(#path_method(#expr_owned_ref))
    }
}
