//! Types for definitions.

use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Fields, Ident, ItemFn, ItemStruct, Type};

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
        let mut items = Vec::new();
        {
            let slice_type_item = self.slice.create_item();
            let owned_type_item = self.owned.create_item();
            items.push(quote! {
                #slice_type_item
                #owned_type_item
            });
        }
        if let Some(validator) = &self.validator {
            let validator_item = validator.create_item();
            items.push(quote! { #validator_item });
        }
        {
            let to_owned_items = self.impl_to_owned();
            items.push(quote! { #to_owned_items });
        }
        {
            let slice_impls = self.impl_derives_for_slice();
            items.extend(slice_impls);
        }
        {
            let owned_impls = self.impl_derives_for_owned();
            items.extend(owned_impls);
        }

        quote! { #(#items)* }
    }

    /// Loads a `Definitions` from the given file content.
    pub(crate) fn from_file(file: syn::File) -> Result<Self, LoadError> {
        Builder::try_from(file)?.build()
    }

    /// Returns the expression converted to a slice type without validation.
    fn slice_inner_to_slice_outer_unchecked(&self, expr: TokenStream) -> TokenStream {
        let ty_slice = self.slice.outer_type();
        let ty_slice_inner = self.slice.inner_type();
        quote! {
            unsafe { &*(#expr as *const #ty_slice_inner as *const #ty_slice) }
        }
    }

    /// Returns the expression converted to an owned type without validation.
    fn owned_inner_to_owned_outer_unchecked(&self, expr: TokenStream) -> TokenStream {
        let ty_owned = self.owned.outer_type();
        let field_owned = self.owned.field_name();
        quote! {
            #ty_owned { #field_owned: #expr }
        }
    }

    /// Implements `Borrowed` and `ToOwned`.
    fn impl_to_owned(&self) -> TokenStream {
        let ty_slice = self.slice.outer_type();
        let ty_slice_inner = self.slice.inner_type();
        let field_slice = self.slice.field_name();
        let ty_owned = self.owned.outer_type();
        let ty_owned_inner = self.owned.inner_type();
        let field_owned = self.owned.field_name();

        let expr_body_borrow = self.slice_inner_to_slice_outer_unchecked(quote! {
            <#ty_owned_inner as std::borrow::Borrow<#ty_slice_inner>>::borrow(&self.#field_owned)
        });
        let expr_body_to_owned = self.owned_inner_to_owned_outer_unchecked(quote! {
            <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&self.#field_slice)
        });

        quote! {
            impl std::borrow::Borrow<#ty_slice> for #ty_owned {
                fn borrow(&self) -> &#ty_slice {
                    #expr_body_borrow
                }
            }

            impl std::borrow::ToOwned for #ty_slice {
                type Owned = #ty_owned;

                fn to_owned(&self) -> Self::Owned {
                    #expr_body_to_owned
                }
            }
        }
    }

    /// Implement traits specified by `#[custom_slice(derive(Foo, Bar))]` for
    /// the slice type.
    fn impl_derives_for_slice<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.slice.attrs.derives().map(move |derive| {
            let derive = derive.to_string();
            match derive.as_str() {
                "Default" => self.impl_slice_default(),
                derive => panic!("Unknown derive target for slice type: {:?}", derive),
            }
        })
    }

    /// Implement traits specified by `#[custom_slice(derive(Foo, Bar))]` for
    /// the owned type.
    fn impl_derives_for_owned<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.owned.attrs.derives().map(|derive| {
            let derive = derive.to_string();
            match derive.as_str() {
                derive => panic!("Unknown derive target for slice type: {:?}", derive),
            }
        })
    }

    /// Implements `Default` for slice type.
    fn impl_slice_default(&self) -> TokenStream {
        let ty_slice = self.slice.outer_type();
        let ty_slice_inner = self.slice.inner_type();

        let expr_body_default = self.slice_inner_to_slice_outer_unchecked(quote! {
            <&#ty_slice_inner as std::default::Default>::default()
        });

        quote! {
            impl std::default::Default for &#ty_slice {
                fn default() -> Self {
                    #expr_body_default
                }
            }
        }
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

    /// Returns the outer type.
    fn outer_type(&self) -> &Ident {
        &self.item.ident
    }

    /// Returns the inner type.
    fn inner_type(&self) -> &Type {
        &self.inner_field.ty
    }

    /// Returns the inner field name or the index.
    fn field_name(&self) -> TokenStream {
        self.inner_field
            .ident
            .as_ref()
            .map_or_else(|| quote! { 0 }, |ident| quote! { #ident })
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
