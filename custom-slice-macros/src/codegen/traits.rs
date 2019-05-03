//! Trait impls.

use quote::{quote, ToTokens};

use crate::{
    codegen::{
        props::{DynMutability, Mutability},
        types::RefType,
    },
    defs::Definitions,
};

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

#[derive(Debug, Clone, Copy)]
pub enum CmpTrait {
    /// `std::cmp::PartialEq`.
    PartialEq,
    /// `std::cmp::PartialOrd`.
    PartialOrd,
}

impl CmpTrait {
    fn trait_path(self) -> impl ToTokens {
        match self {
            CmpTrait::PartialEq => quote!(std::cmp::PartialEq),
            CmpTrait::PartialOrd => quote!(std::cmp::PartialOrd),
        }
    }

    fn method_name(self) -> impl ToTokens {
        match self {
            CmpTrait::PartialEq => quote!(eq),
            CmpTrait::PartialOrd => quote!(partial_cmp),
        }
    }

    fn ty_ret(self) -> impl ToTokens {
        match self {
            CmpTrait::PartialEq => quote!(bool),
            CmpTrait::PartialOrd => quote!(Option<std::cmp::Ordering>),
        }
    }

    pub(crate) fn impl_with_slice(
        self,
        defs: &Definitions,
        lhs: RefType,
        rhs: RefType,
    ) -> impl ToTokens {
        let trait_path = self.trait_path();
        let method = self.method_name();
        let ty_ret = self.ty_ret();
        let arg_rhs = &quote!(__other);

        let ty_slice = defs.ty_slice();
        let expr_lhs = lhs.ref_to_slice_ref(defs, quote!(self));
        let expr_rhs = rhs.ref_to_slice_ref(defs, arg_rhs);
        let ty_lhs = lhs.ty(defs);
        let ty_rhs = rhs.ty(defs);

        quote! {
            impl #trait_path<#ty_rhs> for #ty_lhs {
                fn #method(&self, #arg_rhs: &#ty_rhs) -> #ty_ret {
                    #trait_path::<#ty_slice>::#method(
                        #expr_lhs,
                        #expr_rhs,
                    )
                }
            }
        }
    }
}
