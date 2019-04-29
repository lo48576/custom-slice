//! `Definitions` builder.

use std::{convert::TryFrom, error, fmt, mem::replace};

use syn::Item;

use crate::{
    attrs::{CustomSliceAttrs, SpecialItemType},
    defs::{CustomType, Definitions, Validator},
};

/// Definition load error.
#[derive(Debug)]
pub(crate) enum LoadError {
    /// Extra items found.
    ExtraItems,
    /// No slice definitions found.
    NoSliceDefinitions,
    /// No owned type definitions found.
    NoOwnedDefinitions,
    /// Multiple slice definitions found.
    MultipleSliceDefinitions,
    /// Multiple owned type definitions found.
    MultipleOwnedDefinitions,
    /// Invalid special item.
    InvalidSpecialItem(SpecialItemType),
    /// There are unexpected number of fields.
    UnexpectedFields(usize),
}

impl error::Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::ExtraItems => write!(f, "Extra items found"),
            LoadError::NoSliceDefinitions => write!(f, "No slice definitions found"),
            LoadError::NoOwnedDefinitions => write!(f, "No owned type definitions found"),
            LoadError::MultipleSliceDefinitions => write!(f, "Multiple slice definitions found"),
            LoadError::MultipleOwnedDefinitions => {
                write!(f, "Multiple owned type definitions found")
            }
            LoadError::InvalidSpecialItem(ty) => write!(f, "Invalid special item: {:?}", ty),
            LoadError::UnexpectedFields(num) => write!(
                f,
                "UnexpectedFields number of fields: expect just one, but got {}",
                num
            ),
        }
    }
}

#[derive(Default)]
pub(crate) struct Builder {
    /// Slice type definition.
    slice: Option<CustomType>,
    /// Owned type definition.
    owned: Option<CustomType>,
    /// Validator function definition.
    validator: Option<Validator>,
}

impl Builder {
    /// Builds a `Definitions`.
    pub(crate) fn build(self) -> Result<Definitions, LoadError> {
        let slice = self.slice.ok_or(LoadError::NoSliceDefinitions)?;
        let owned = self.owned.ok_or(LoadError::NoOwnedDefinitions)?;

        Ok(Definitions {
            slice,
            owned,
            validator: self.validator,
        })
    }
}

impl TryFrom<syn::File> for Builder {
    type Error = LoadError;

    fn try_from(file: syn::File) -> Result<Self, Self::Error> {
        let mut builder = Self::default();

        for item in file.items {
            match item {
                Item::Fn(mut item_fn) => {
                    let attrs = CustomSliceAttrs::from(replace(&mut item_fn.attrs, Vec::new()));
                    match attrs.special_item_type() {
                        Some(SpecialItemType::Validator) => {
                            let validator = Validator {
                                item: item_fn,
                                attrs,
                            };
                            if builder.validator.replace(validator).is_some() {
                                return Err(LoadError::MultipleSliceDefinitions);
                            }
                        }
                        Some(ty) => return Err(LoadError::InvalidSpecialItem(ty)),
                        None => return Err(LoadError::ExtraItems),
                    }
                }
                Item::Struct(mut item_struct) => {
                    let attrs = CustomSliceAttrs::from(replace(&mut item_struct.attrs, Vec::new()));
                    match attrs.special_item_type() {
                        Some(SpecialItemType::SliceType) => {
                            let def = CustomType::new(item_struct, attrs)?;
                            if builder.slice.replace(def).is_some() {
                                return Err(LoadError::MultipleSliceDefinitions);
                            }
                        }
                        Some(SpecialItemType::OwnedType) => {
                            let def = CustomType::new(item_struct, attrs)?;
                            if builder.owned.replace(def).is_some() {
                                return Err(LoadError::MultipleOwnedDefinitions);
                            }
                        }
                        Some(ty) => return Err(LoadError::InvalidSpecialItem(ty)),
                        None => return Err(LoadError::ExtraItems),
                    }
                }
                _ => return Err(LoadError::ExtraItems),
            }
        }

        Ok(builder)
    }
}
