//! Types for definitions.

use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
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
            let slice_methods = self.impl_methods_for_slice();
            let owned_methods = self.impl_methods_for_owned();
            items.push(quote! {
                #slice_methods
                #owned_methods
            });
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

    /// Implements methods for the slice type.
    fn impl_methods_for_slice(&self) -> TokenStream {
        let new_unchecked =
            self.impl_slice_constructor_unchecked("new_unchecked", Mutability::Constant);
        let new_unchecked_mut =
            self.impl_slice_constructor_unchecked("new_unchecked_mut", Mutability::Mutable);
        let new_checked = self.impl_slice_constructor_checked("new_checked", Mutability::Constant);
        let new_checked_mut =
            self.impl_slice_constructor_checked("new_checked_mut", Mutability::Mutable);

        if new_unchecked.is_some()
            || new_unchecked_mut.is_some()
            || new_checked.is_some()
            || new_checked_mut.is_some()
        {
            let ty_slice = self.slice.outer_type();
            quote! {
                impl #ty_slice {
                    #new_unchecked
                    #new_unchecked_mut
                    #new_checked
                    #new_checked_mut
                }
            }
        } else {
            quote! {}
        }
    }

    fn impl_slice_constructor_unchecked(
        &self,
        attr_name: &str,
        mutability: Mutability,
    ) -> Option<ItemFn> {
        let arg_name = &quote! { _v };
        let ty_slice_inner_ref = mutability.make_ref(self.slice.inner_type());
        let ty_slice_ref = mutability.make_ref(self.slice.outer_type());

        let fn_prefix = self.slice.attrs.get_constructor(attr_name)?;
        let mut new_fn = fn_prefix
            .build_item(arg_name, ty_slice_inner_ref, ty_slice_ref, quote! {})
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        let block = self.slice.slice_inner_to_outer_unchecked(
            arg_name,
            new_fn.unsafety.is_some(),
            mutability,
        );
        *new_fn.block = syn::parse2(quote! {{ #block }}).expect("Should never fail: valid block");
        Some(new_fn)
    }

    fn impl_slice_constructor_checked(
        &self,
        attr_name: &str,
        mutability: Mutability,
    ) -> Option<ItemFn> {
        let fn_prefix = self.slice.attrs.get_constructor(attr_name)?;

        let arg_name = &quote! { _v };
        let error_var = &quote! { _e };

        let ty_error = self
            .slice
            .attrs
            .get_error_type()
            .expect("Failed to parse error type")
            .expect("`#[custom_slice(error(type = \"...\"))]` should be specified");
        let mapped_error = self
            .slice
            .attrs
            .get_map_error(error_var, arg_name)
            .expect("Failed to parse `map_error`")
            .map(|map| quote! { #map })
            .unwrap_or_else(|| error_var.clone());

        let ty_slice_inner_ref = mutability.make_ref(self.slice.inner_type());
        let ty_slice_ref = mutability.make_ref(self.slice.outer_type());

        let validator = match &self.validator {
            Some(v) => v,
            None => panic!(
                "Validator should be necessary for checked constructor: attr_name = {:?}",
                attr_name
            ),
        };

        let mut new_fn = fn_prefix
            .build_item(
                arg_name,
                ty_slice_inner_ref,
                quote! { std::result::Result<#ty_slice_ref, #ty_error> },
                quote! {},
            )
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        let unsafe_expr = self.slice.slice_inner_to_outer_unchecked(
            arg_name,
            new_fn.unsafety.is_some(),
            mutability,
        );
        let validate_fn = validator.name();
        let block = quote! {{
            match #validate_fn(#arg_name) {
                Ok(_) => Ok(#unsafe_expr),
                Err(#error_var) => Err(#mapped_error),
            }
        }};
        *new_fn.block = syn::parse2(block).expect("Should never fail: valid block");
        Some(new_fn)
    }

    /// Implements methods for the owned type.
    fn impl_methods_for_owned(&self) -> TokenStream {
        let new_unchecked = self.impl_owned_constructor_unchecked("new_unchecked");
        let new_checked = self.impl_owned_constructor_checked("new_checked");

        if new_unchecked.is_some() || new_checked.is_some() {
            let ty_owned = self.owned.outer_type();
            quote! {
                impl #ty_owned {
                    #new_unchecked
                    #new_checked
                }
            }
        } else {
            quote! {}
        }
    }

    fn impl_owned_constructor_unchecked(&self, attr_name: &str) -> Option<ItemFn> {
        let fn_prefix = self.owned.attrs.get_constructor(attr_name)?;

        let ty_owned_inner = self.owned.inner_type();
        let arg_name = &quote! { _v };
        let new_fn = fn_prefix
            .build_item(
                arg_name,
                ty_owned_inner,
                quote! { Self },
                self.owned.owned_inner_to_outer_unchecked(arg_name),
            )
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        Some(new_fn)
    }

    fn impl_owned_constructor_checked(&self, attr_name: &str) -> Option<ItemFn> {
        let fn_prefix = self.owned.attrs.get_constructor(attr_name)?;

        let arg_name = &quote! { _v };
        let error_var = &quote! { _e };

        let ty_error = self
            .owned
            .attrs
            .get_error_type()
            .expect("Failed to parse error type")
            .expect("`#[custom_slice(error(type = \"...\"))]` should be specified");
        let mapped_error = self
            .owned
            .attrs
            .get_map_error(error_var, arg_name)
            .expect("Failed to parse `map_error`")
            .map(|map| quote! { #map })
            .unwrap_or_else(|| error_var.clone());

        let ty_owned_inner = self.owned.inner_type();

        let validator = match &self.validator {
            Some(v) => v,
            None => panic!(
                "Validator should be necessary for checked constructor: attr_name = {:?}",
                attr_name
            ),
        };

        let mut new_fn = fn_prefix
            .build_item(
                arg_name,
                ty_owned_inner,
                quote! { std::result::Result<Self, #ty_error> },
                quote! {},
            )
            .unwrap_or_else(|e| panic!("Failed to parse `{}` attribute: {}", attr_name, e));
        let val_expr = self.owned.owned_inner_to_outer_unchecked(arg_name);
        let validate_fn = validator.name();
        let ty_slice_inner = self.slice.inner_type();
        let block = quote! {{
            match #validate_fn(
                <#ty_owned_inner as std::borrow::Borrow<#ty_slice_inner>>::borrow(&#arg_name)
            ) {
                Ok(_) => Ok(#val_expr),
                Err(#error_var) => Err(#mapped_error),
            }
        }};
        *new_fn.block = syn::parse2(block).expect("Should never fail: valid block");
        Some(new_fn)
    }

    /// Implements `Borrowed` and `ToOwned`.
    fn impl_to_owned(&self) -> TokenStream {
        let ty_slice = self.slice.outer_type();
        let ty_slice_inner = self.slice.inner_type();
        let expr_slice_inner = self.slice.inner_expr(quote! { self });
        let ty_owned = self.owned.outer_type();
        let ty_owned_inner = self.owned.inner_type();
        let expr_owned_inner = self.owned.inner_expr(quote! { self });

        let expr_body_borrow = self.slice.slice_inner_to_outer_unchecked(quote! {
            <#ty_owned_inner as std::borrow::Borrow<#ty_slice_inner>>::borrow(&#expr_owned_inner)
        }, false, Mutability::Constant);
        let expr_body_to_owned = self.owned.owned_inner_to_outer_unchecked(quote! {
            <#ty_slice_inner as std::borrow::ToOwned>::to_owned(&#expr_slice_inner)
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
                "DefaultRef" => self.impl_slice_default(false),
                "DefaultRefMut" => self.impl_slice_default(true),
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
    fn impl_slice_default(&self, mutable: bool) -> TokenStream {
        let ty_slice = self.slice.outer_type();
        let ty_slice_inner = self.slice.inner_type();

        let mut_ = if mutable { Some(quote! { mut }) } else { None };
        let default = quote! {
            <&#mut_ #ty_slice_inner as std::default::Default>::default()
        };
        let expr_body_default = if mutable {
            self.slice
                .slice_inner_to_outer_unchecked(default, false, Mutability::Mutable)
        } else {
            self.slice
                .slice_inner_to_outer_unchecked(default, false, Mutability::Constant)
        };

        quote! {
            impl std::default::Default for &#mut_ #ty_slice {
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

    /// Returns the inner type expression from the outer type expression
    pub(crate) fn inner_expr(&self, outer_expr: impl ToTokens) -> TokenStream {
        let field_name = self.field_name();
        quote! { #outer_expr.#field_name }
    }

    /// Returns the expression converted to a slice type without validation.
    fn slice_inner_to_outer_unchecked(
        &self,
        expr: impl ToTokens,
        is_unsafe_context: bool,
        mutability: Mutability,
    ) -> TokenStream {
        let ty_slice_inner_ptr = mutability.make_ptr(self.inner_type());
        let ty_slice_ptr = mutability.make_ptr(self.outer_type());
        // Type: &#ty_slice
        let base = mutability.make_ref(quote! {
            *(#expr as #ty_slice_inner_ptr as #ty_slice_ptr)
        });
        if is_unsafe_context {
            base
        } else {
            quote! { unsafe { #base } }
        }
    }

    /// Returns the expression converted to an owned type without validation.
    fn owned_inner_to_outer_unchecked(&self, expr: impl ToTokens) -> TokenStream {
        let ty_owned = self.outer_type();
        let field_owned = self.field_name();
        // Type: #ty_owned
        quote! {
            #ty_owned { #field_owned: #expr }
        }
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

    /// Returns function name.
    fn name(&self) -> &Ident {
        &self.item.ident
    }
}

/// Mutability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mutability {
    /// Mutable.
    Mutable,
    /// Constant.
    Constant,
}

impl Mutability {
    fn make_ref(self, following: impl ToTokens) -> TokenStream {
        match self {
            Mutability::Mutable => quote! { &mut #following },
            Mutability::Constant => quote! { &#following },
        }
    }

    fn make_ptr(self, following: impl ToTokens) -> TokenStream {
        match self {
            Mutability::Mutable => quote! { *mut #following },
            Mutability::Constant => quote! { *const #following },
        }
    }
}
