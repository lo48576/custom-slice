//! Types for definitions.

use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, Fields, Ident, ItemFn, ItemStruct, Type};

use crate::{
    attrs::CustomSliceAttrs,
    codegen::{
        expr::{OwnedInner, SliceInner},
        props::{Constant, Mutability, Mutable, Safety},
        traits,
    },
};

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
        let mut tokens = TokenStream::new();

        // Type definitions.
        self.slice.create_item().to_tokens(&mut tokens);
        self.owned.create_item().to_tokens(&mut tokens);
        // Validator function definition.
        if let Some(validator) = &self.validator {
            validator.create_item().to_tokens(&mut tokens);
        }

        // Methods for slice type.
        self.impl_methods_for_slice().to_tokens(&mut tokens);
        // Methods for owned type.
        self.impl_methods_for_owned().to_tokens(&mut tokens);

        // `ToOwned` for slice type.
        traits::slice::impl_to_owned(self).to_tokens(&mut tokens);
        // `Borrow` for owned type.
        traits::owned::impl_borrow(self, Constant).to_tokens(&mut tokens);

        // std trait impls for slice types.
        self.impl_derives_for_slice()
            .for_each(|v| v.to_tokens(&mut tokens));
        // std trait impls for owned types.
        self.impl_derives_for_owned()
            .for_each(|v| v.to_tokens(&mut tokens));

        tokens
    }

    /// Loads a `Definitions` from the given file content.
    pub(crate) fn from_file(file: syn::File) -> Result<Self, LoadError> {
        Builder::try_from(file)?.build()
    }

    pub(crate) fn slice(&self) -> &CustomType {
        &self.slice
    }

    pub(crate) fn owned(&self) -> &CustomType {
        &self.owned
    }

    /// Implements methods for the slice type.
    fn impl_methods_for_slice(&self) -> Option<TokenStream> {
        let mut body = TokenStream::new();
        self.impl_slice_constructor_unchecked("new_unchecked", Constant)
            .to_tokens(&mut body);
        self.impl_slice_constructor_unchecked("new_unchecked_mut", Mutable)
            .to_tokens(&mut body);
        self.impl_slice_constructor_checked("new_checked", Constant)
            .to_tokens(&mut body);
        self.impl_slice_constructor_checked("new_checked_mut", Mutable)
            .to_tokens(&mut body);

        if body.is_empty() {
            return None;
        }
        let ty_slice = self.slice.outer_type();
        Some(quote! { impl #ty_slice { #body } })
    }

    fn impl_slice_constructor_unchecked(
        &self,
        attr_name: &str,
        mutability: impl Mutability,
    ) -> Option<ItemFn> {
        let arg_name = SliceInner::new(quote! { _v }, mutability);
        let ty_slice_inner_ref = mutability.make_ref(self.slice.inner_type());
        let ty_slice_ref = mutability.make_ref(self.slice.outer_type());

        let fn_prefix = self.slice.attrs.get_constructor(attr_name)?;
        let mut new_fn = fn_prefix
            .build_item(&arg_name, ty_slice_inner_ref, ty_slice_ref, quote! {})
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        let block = arg_name.to_slice_unchecked(self, Safety::from(&new_fn.unsafety));
        *new_fn.block = syn::parse2(quote! {{ #block }}).expect("Should never fail: valid block");
        Some(new_fn)
    }

    fn impl_slice_constructor_checked(
        &self,
        attr_name: &str,
        mutability: impl Mutability,
    ) -> Option<ItemFn> {
        let fn_prefix = self.slice.attrs.get_constructor(attr_name)?;
        let arg_name = SliceInner::new(quote! { _v }, mutability);
        let error_var = &quote! { _e };

        let (ty_error, mapped_error) =
            get_error_ty_and_val(&self.slice.attrs, error_var, &arg_name);

        let ty_slice_ref = mutability.make_ref(self.slice.outer_type());
        let mut new_fn = fn_prefix
            .build_item(
                &arg_name,
                mutability.make_ref(self.slice.inner_type()),
                quote! { std::result::Result<#ty_slice_ref, #ty_error> },
                quote! {},
            )
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        let block = {
            let expr_outer = arg_name.to_slice_unchecked(self, Safety::from(&new_fn.unsafety));
            let fn_validate = match &self.validator {
                Some(v) => v.name(),
                None => panic!(
                    "Validator should be necessary for checked constructor: attr_name = {:?}",
                    attr_name
                ),
            };
            quote! {{
                match #fn_validate(#arg_name) {
                    Ok(_) => Ok(#expr_outer),
                    Err(#error_var) => Err(#mapped_error),
                }
            }}
        };
        *new_fn.block = syn::parse2(block).expect("Should never fail: valid block");
        Some(new_fn)
    }

    /// Implements methods for the owned type.
    fn impl_methods_for_owned(&self) -> Option<TokenStream> {
        let mut body = TokenStream::new();
        self.impl_owned_constructor_unchecked("new_unchecked")
            .to_tokens(&mut body);
        self.impl_owned_constructor_checked("new_checked")
            .to_tokens(&mut body);

        if body.is_empty() {
            return None;
        }
        let ty_owned = self.owned.outer_type();
        Some(quote! { impl #ty_owned { #body } })
    }

    fn impl_owned_constructor_unchecked(&self, attr_name: &str) -> Option<ItemFn> {
        let fn_prefix = self.owned.attrs.get_constructor(attr_name)?;

        let ty_owned_inner = self.owned.inner_type();
        let arg_name = OwnedInner::new(quote! { _v });
        let new_fn = fn_prefix
            .build_item(
                &arg_name,
                ty_owned_inner,
                quote! { Self },
                arg_name.to_owned_unchecked(self),
            )
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        Some(new_fn)
    }

    fn impl_owned_constructor_checked(&self, attr_name: &str) -> Option<ItemFn> {
        let fn_prefix = self.owned.attrs.get_constructor(attr_name)?;
        let arg_name = OwnedInner::new(quote! { _v });
        let error_var = &quote! { _e };

        let (ty_error, mapped_error) =
            get_error_ty_and_val(&self.owned.attrs, error_var, &arg_name);

        let block = {
            let val_expr = arg_name.to_owned_unchecked(self);
            let expr_slice_inner_ref = OwnedInner::new(&arg_name).to_slice_inner_ref(self);
            let fn_validate = match &self.validator {
                Some(v) => v.name(),
                None => panic!(
                    "Validator should be necessary for checked constructor: attr_name = {:?}",
                    attr_name
                ),
            };
            quote! {{
                match #fn_validate(#expr_slice_inner_ref) {
                    Ok(_) => Ok(#val_expr),
                    Err(#error_var) => Err(#mapped_error),
                }
            }}
        };
        let new_fn = fn_prefix
            .build_item(
                arg_name,
                self.owned.inner_type(),
                quote! { std::result::Result<Self, #ty_error> },
                block,
            )
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        Some(new_fn)
    }

    /// Implement traits specified by `#[custom_slice(derive(Foo, Bar))]` for
    /// the slice type.
    fn impl_derives_for_slice<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.slice.attrs.derives().map(move |derive| {
            let derive = derive.to_string();
            match derive.as_str() {
                "DefaultRef" => traits::slice::impl_default_ref(self, Constant),
                "DefaultRefMut" => traits::slice::impl_default_ref(self, Mutable),
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
        ItemStruct {
            attrs: self.attrs.raw.clone(),
            ..self.item.clone()
        }
    }

    /// Returns the outer type.
    pub(crate) fn outer_type(&self) -> &Ident {
        &self.item.ident
    }

    /// Returns the inner type.
    pub(crate) fn inner_type(&self) -> &Type {
        &self.inner_field.ty
    }

    /// Returns the inner field name or the index.
    pub(crate) fn field_name(&self) -> TokenStream {
        self.inner_field
            .ident
            .as_ref()
            .map_or_else(|| quote! { 0 }, |ident| quote! { #ident })
    }

    /// Returns the inner type expression from the outer type expression
    pub(crate) fn inner_expr(&self, outer_expr: impl ToTokens) -> TokenStream {
        let field_name = self.field_name();
        quote! { #outer_expr.#field_name }
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
        ItemFn {
            attrs: self.attrs.raw.clone(),
            ..self.item.clone()
        }
    }

    /// Returns function name.
    fn name(&self) -> &Ident {
        &self.item.ident
    }
}

fn get_error_ty_and_val(
    attrs: &CustomSliceAttrs,
    error_var: impl ToTokens,
    arg_name: impl ToTokens,
) -> (syn::Type, TokenStream) {
    let ty_error = attrs
        .get_error_type()
        .expect("Failed to parse error type")
        .expect("`#[custom_slice(error(type = \"...\"))]` should be specified");
    let mapped_error = attrs
        .get_mapped_error(error_var, arg_name)
        .expect("Failed to parse `map_error`");
    (ty_error, mapped_error)
}