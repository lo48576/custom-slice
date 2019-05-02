//! Attributes.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Expr, Ident, ItemFn, Lit, Meta, MetaNameValue, NestedMeta, Type};

/// Special item types.
#[derive(Debug, Clone, Copy)]
pub(crate) enum SpecialItemType {
    /// Slice type definition.
    SliceType,
    /// Owned type definition.
    OwnedType,
    /// Validator function definition.
    Validator,
}

impl SpecialItemType {
    fn from_ident(ident: &Ident) -> Option<Self> {
        if ident == "slice" {
            Some(SpecialItemType::SliceType)
        } else if ident == "owned" {
            Some(SpecialItemType::OwnedType)
        } else if ident == "validator" {
            Some(SpecialItemType::Validator)
        } else {
            None
        }
    }
}

/// Meta for custom slice items.
pub(crate) struct CustomSliceAttrs {
    /// Custom meta.
    pub(crate) custom_meta: Vec<NestedMeta>,
    /// Raw attributes (not for custom-slice).
    pub(crate) raw: Vec<Attribute>,
}

impl CustomSliceAttrs {
    /// Checks whether the type is slice type.
    pub(crate) fn special_item_type(&self) -> Option<SpecialItemType> {
        for meta in &self.custom_meta {
            if let NestedMeta::Meta(Meta::Word(ident)) = meta {
                let ty = SpecialItemType::from_ident(ident);
                if ty.is_some() {
                    return ty;
                }
            }
        }

        None
    }

    /// Returns `[foo, bar, ..]` of `#[custom_slice(name(foo, bar, ..))]`.
    fn lists<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a NestedMeta> + 'a {
        self.custom_meta
            .iter()
            .filter_map(move |nested_meta| match nested_meta {
                NestedMeta::Meta(Meta::List(list)) if list.ident == name => Some(list),
                _ => None,
            })
            .flat_map(|list| &list.nested)
    }

    /// Returns `foo=bar, ..` of `#[custom_slice(foo = bar)]`.
    fn namevalues<'a>(&'a self) -> impl Iterator<Item = &'a MetaNameValue> + 'a {
        self.custom_meta
            .iter()
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(Meta::NameValue(meta)) => Some(meta),
                _ => None,
            })
    }

    /// Checks whether `#[repr(transparent)]` or `#[repr(C)]` is specified.
    pub(crate) fn is_repr_transparent_or_c(&self) -> bool {
        self.raw
            .iter()
            .filter_map(|attr| attr.parse_meta().ok())
            .filter_map(|meta| match meta {
                Meta::List(list) => Some(list),
                _ => None,
            })
            .filter(|list| list.ident == "repr")
            .flat_map(|list| list.nested)
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(Meta::Word(ident)) => Some(ident),
                _ => None,
            })
            .any(|ident| ident == "transparent" || ident == "C")
    }

    /// Returns an iterator of identifiers to be `derive`d.
    pub(crate) fn derives<'a>(&'a self) -> impl Iterator<Item = &'a Ident> + 'a {
        self.lists("derive")
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(Meta::Word(ident)) => Some(ident),
                _ => None,
            })
    }

    /// Returns value part of name-value meta.
    fn get_nv_value<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Lit> + 'a {
        self.namevalues()
            .filter(move |nv| nv.ident == name)
            .map(|nv| &nv.lit)
    }

    pub(crate) fn get_fn_prefix(&self, attr_name: &str) -> Option<FnPrefix> {
        self.get_nv_value(attr_name)
            .filter_map(|lit| match lit {
                Lit::Str(ref s) => Some(FnPrefix::from(s.value())),
                _ => None,
            })
            .next()
    }

    fn get_error_conf<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a Lit> + 'a {
        self.lists("error")
            .filter_map(move |nested_meta| match nested_meta {
                NestedMeta::Meta(Meta::NameValue(nv)) if nv.ident == key => Some(&nv.lit),
                _ => None,
            })
    }

    pub(crate) fn get_error_type(&self) -> Result<Option<Type>, syn::Error> {
        self.get_error_conf("type")
            .filter_map(|lit| match lit {
                Lit::Str(ref s) => Some(s.parse::<Type>()),
                _ => None,
            })
            .next()
            .transpose()
    }

    pub(crate) fn get_mapped_error(
        &self,
        error_var: impl ToTokens,
        arg_name: impl ToTokens,
    ) -> Result<TokenStream, syn::Error> {
        let error_var_s = (&error_var).into_token_stream().to_string();
        let arg_name = arg_name.into_token_stream().to_string();
        self.get_error_conf("map")
            .filter_map(|lit| match lit {
                Lit::Str(ref s) => Some(syn::parse_str::<Expr>(&format!(
                    "{}({}, {})",
                    s.value(),
                    error_var_s,
                    arg_name,
                ))),
                _ => None,
            })
            .next()
            .transpose()
            .map(|toks| match toks {
                Some(v) => v.into_token_stream(),
                None => error_var.into_token_stream(),
            })
    }
}

impl From<Vec<Attribute>> for CustomSliceAttrs {
    fn from(attrs: Vec<Attribute>) -> Self {
        let mut raw = Vec::new();
        let mut custom = Vec::new();
        for attr in attrs {
            let meta = match attr.parse_meta() {
                Ok(v) => v,
                Err(_) => {
                    raw.push(attr);
                    continue;
                }
            };
            if let Meta::List(list) = meta {
                if list.ident == "custom_slice" {
                    custom.extend(list.nested);
                    continue;
                }
            }
            raw.push(attr);
        }

        Self {
            custom_meta: custom,
            raw,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnPrefix {
    /// Function definition without `(args...) -> Type { body }` part.
    prefix: String,
}

impl FnPrefix {
    // If you want to omit argument type, pass an empty token stream.
    pub(crate) fn build_item(
        &self,
        arg_name: impl ToTokens,
        ty_arg: impl ToTokens,
        ty_ret: impl ToTokens,
        body_expr: impl ToTokens,
    ) -> Result<ItemFn, syn::Error> {
        let ty_arg = ty_arg.into_token_stream();
        let arg_part = if ty_arg.is_empty() {
            arg_name.into_token_stream()
        } else {
            quote!(#arg_name: #ty_arg)
        };
        let following = quote! {
            (#arg_part) -> #ty_ret { #body_expr }
        };
        syn::parse_str::<ItemFn>(&format!("{}{}", self.prefix, following.to_string()))
    }
}

impl From<String> for FnPrefix {
    fn from(prefix: String) -> Self {
        Self { prefix }
    }
}
