//! Custom string test.

use std::{error, fmt};

/// Error for lower ascii string creation.
#[derive(Debug, Clone, Copy)]
pub struct Error(char);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Non-ascii-lowercase character: {:?}", self.0)
    }
}

impl error::Error for Error {}

custom_slice_macros::define_slice_types_pair! {
    /// A string which contains only lower ascii characters.
    #[custom_slice(owned)]
    #[derive(Default)]
    pub struct LowerAsciiString(String);

    /// A string which contains only lower ascii characters.
    #[custom_slice(slice)]
    #[custom_slice(derive(Default))]
    #[repr(transparent)]
    pub struct LowerAsciiStr(str);

    /// Validates that the given string as `LowerAsciiStr`.
    #[custom_slice(validator)]
    fn validate(s: &str) -> Result<&str, Error> {
        match s.chars().find(|c| !c.is_ascii_lowercase()) {
            Some(c) => return Err(Error(c)),
            None => Ok(s),
        }
    }
}

#[test]
fn default_string() {
    let _ = LowerAsciiString::default();
}

#[test]
fn default_str() {
    let _ = <&LowerAsciiStr>::default();
}

#[test]
fn string_conversion() {
    use std::borrow::{Borrow, ToOwned};

    let string = LowerAsciiString::default();
    let s: &LowerAsciiStr = string.borrow();
    let _: LowerAsciiString = s.to_owned();
}
