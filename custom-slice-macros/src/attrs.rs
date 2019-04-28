//! Attributes.

use quote::ToTokens;
use syn::{Attribute, Expr, Ident, ItemFn, Lit, Meta, NestedMeta, Type};

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

    fn get_sublevel_meta<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a NestedMeta> + 'a {
        self.custom_meta
            .iter()
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            })
            .filter_map(|meta| match meta {
                Meta::List(list) => Some(list),
                _ => None,
            })
            .filter(move |list| list.ident == name)
            .flat_map(|list| list.nested.iter())
    }

    /// Returns an iterator of identifiers to be `derive`d.
    pub(crate) fn derives<'a>(&'a self) -> impl Iterator<Item = &'a Ident> + 'a {
        self.get_sublevel_meta("derive")
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            })
            .filter_map(|meta| match meta {
                Meta::Word(ident) => Some(ident),
                _ => None,
            })
    }

    /// Returns value part of name-value meta.
    fn get_nv_value<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Lit> + 'a {
        self.custom_meta
            .iter()
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            })
            .filter_map(|meta| match meta {
                Meta::NameValue(nv) => Some(nv),
                _ => None,
            })
            .filter(move |nv| nv.ident == name)
            .map(|nv| &nv.lit)
    }

    fn get_item_fn_from_fn_prefix(&self, name: &str) -> Result<Option<ItemFn>, syn::Error> {
        self.get_nv_value(name)
            .filter_map(|lit| match lit {
                Lit::Str(ref s) => Some(syn::parse_str::<ItemFn>(&format!(
                    "{}() -> () {{}}",
                    s.value()
                ))),
                _ => None,
            })
            .next()
            .transpose()
    }

    /// Returns `new_unchecked` value.
    pub(crate) fn get_new_unchecked(&self) -> Result<Option<ItemFn>, syn::Error> {
        self.get_item_fn_from_fn_prefix("new_unchecked")
    }

    /// Returns `new_unchecked_mut` value.
    pub(crate) fn get_new_unchecked_mut(&self) -> Result<Option<ItemFn>, syn::Error> {
        self.get_item_fn_from_fn_prefix("new_unchecked_mut")
    }

    /// Returns `new_checked` value.
    pub(crate) fn get_new_checked(&self) -> Result<Option<ItemFn>, syn::Error> {
        self.get_item_fn_from_fn_prefix("new_checked")
    }

    /// Returns `new_checked_mut` value.
    pub(crate) fn get_new_checked_mut(&self) -> Result<Option<ItemFn>, syn::Error> {
        self.get_item_fn_from_fn_prefix("new_checked_mut")
    }

    fn get_error_conf<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a Lit> + 'a {
        self.get_sublevel_meta("error")
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            })
            .filter_map(|meta| match meta {
                Meta::NameValue(nv) => Some(nv),
                _ => None,
            })
            .filter(move |nv| nv.ident == key)
            .map(|nv| &nv.lit)
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

    pub(crate) fn get_map_error(
        &self,
        error_var: impl ToTokens,
        arg_name: impl ToTokens,
    ) -> Result<Option<Expr>, syn::Error> {
        let error_var = error_var.into_token_stream().to_string();
        let arg_name = arg_name.into_token_stream().to_string();
        self.get_error_conf("map")
            .filter_map(|lit| match lit {
                Lit::Str(ref s) => Some(syn::parse_str::<Expr>(&format!(
                    "{}({}, {})",
                    s.value(),
                    error_var,
                    arg_name,
                ))),
                _ => None,
            })
            .next()
            .transpose()
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
