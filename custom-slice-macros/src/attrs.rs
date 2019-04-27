//! Attributes.

use syn::{Attribute, Ident, Meta, NestedMeta};

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
