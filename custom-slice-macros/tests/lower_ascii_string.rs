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
    #[custom_slice(new_unchecked = "unsafe fn new_unchecked")]
    #[custom_slice(new_checked = "pub fn new")]
    #[custom_slice(error(type = "Error"))]
    pub struct LowerAsciiString(String);

    /// A string which contains only lower ascii characters.
    #[custom_slice(slice)]
    #[custom_slice(derive(DefaultRef, DefaultRefMut))]
    #[repr(transparent)]
    #[custom_slice(new_unchecked = "unsafe fn new_unchecked")]
    #[custom_slice(new_unchecked_mut = "unsafe fn new_unchecked_mut")]
    #[custom_slice(new_checked = "pub fn new")]
    #[custom_slice(new_checked_mut = "pub fn new_mut")]
    #[custom_slice(error(type = "Error"))]
    pub struct LowerAsciiStr(str);

    /// Validates that the given string as `LowerAsciiStr`.
    #[custom_slice(validator)]
    fn validate(s: &str) -> Result<(), Error> {
        match s.chars().find(|c| !c.is_ascii_lowercase()) {
            Some(c) => Err(Error(c)),
            None => Ok(()),
        }
    }
}

#[test]
fn default() {
    let _ = LowerAsciiString::default();
    let _ = <&LowerAsciiStr>::default();
}

#[test]
fn new() {
    {
        let res: Result<LowerAsciiString, Error> = LowerAsciiString::new("hello".to_owned());
        assert!(res.is_ok());
    }
    {
        let res: Result<&LowerAsciiStr, Error> = LowerAsciiStr::new("hello");
        assert!(res.is_ok());
    }
    {
        let mut hello = "hello".to_owned();
        let hello_mut: &mut str = &mut hello;
        let res: Result<&mut LowerAsciiStr, Error> = LowerAsciiStr::new_mut(hello_mut);
        assert!(res.is_ok());
    }
}

#[test]
fn new_should_fail() {
    assert!(LowerAsciiString::new("Hello".to_owned()).is_err());
    assert!(LowerAsciiStr::new("Hello").is_err());
}

#[test]
fn new_unchecked() {
    let _: LowerAsciiString = unsafe { LowerAsciiString::new_unchecked("hello".to_owned()) };
    let _: &LowerAsciiStr = unsafe { LowerAsciiStr::new_unchecked("hello") };
    {
        let mut hello = "hello".to_owned();
        let hello_mut: &mut str = &mut hello;
        let _: &mut LowerAsciiStr = unsafe { LowerAsciiStr::new_unchecked_mut(hello_mut) };
    }
}

#[test]
fn borrow_and_to_owned() {
    use std::borrow::{Borrow, ToOwned};

    let string = LowerAsciiString::default();
    let s: &LowerAsciiStr = string.borrow();
    let _: LowerAsciiString = s.to_owned();
}
