//! Types for definitions.

use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Fields, ItemFn, ItemStruct};

use crate::attrs::CustomSliceAttrs;

use self::builder::{Builder, LoadError};

mod builder;

/// Definitions.
pub(crate) struct Definitions {
    /// Slice type definition.
    slice: CustomType,
    /// Owned type definition.
    owned: CustomType,
    /// Validator function definition.
    validator: Option<Validator>,
}

impl Definitions {
    /// Generate tokens.
    pub(crate) fn generate(&self) -> TokenStream {
        let slice_type_item = self.slice.create_item();
        let owned_type_item = self.owned.create_item();
        let validator_item = self.validator.as_ref().map(Validator::create_item);
        quote! {
            #slice_type_item
            #owned_type_item
            #validator_item
        }
    }

    /// Loads a `Definitions` from the given file content.
    pub(crate) fn from_file(file: syn::File) -> Result<Self, LoadError> {
        Builder::try_from(file)?.build()
    }
}

/// Custom type definition.
pub(crate) struct CustomType {
    /// Item.
    item: ItemStruct,
    /// Attributes.
    attrs: CustomSliceAttrs,
    /// Inner field.
    inner_field: Field,
}

impl CustomType {
    /// Creates a new `CustomType`.
    fn new(item: ItemStruct, attrs: CustomSliceAttrs) -> Result<Self, LoadError> {
        // Check number of fields.
        let inner_field = match &item.fields {
            Fields::Named(fields) => {
                if fields.named.len() != 1 {
                    return Err(LoadError::UnexpectedFields(fields.named.len()));
                }
                fields.named[0].clone()
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return Err(LoadError::UnexpectedFields(fields.unnamed.len()));
                }
                fields.unnamed[0].clone()
            }
            Fields::Unit => return Err(LoadError::UnexpectedFields(0)),
        };

        Ok(Self {
            item,
            attrs,
            inner_field,
        })
    }

    /// Creates an item.
    fn create_item(&self) -> ItemStruct {
        let mut item = self.item.clone();
        item.attrs = self.attrs.raw.clone();
        item
    }
}

/// Validator specification.
pub(crate) struct Validator {
    /// Item.
    item: ItemFn,
    /// Attributes.
    attrs: CustomSliceAttrs,
}

impl Validator {
    /// Creates an item.
    fn create_item(&self) -> ItemFn {
        let mut item = self.item.clone();
        item.attrs = self.attrs.raw.clone();
        item
    }
}
