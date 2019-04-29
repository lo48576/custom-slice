//! Custom string test.

use std::{error, fmt};

/// Error for UTF-8 string slice creation.
///
/// See <https://doc.rust-lang.org/stable/std/str/struct.Utf8Error.html>.
#[derive(Debug, Clone, Copy)]
pub struct Utf8Error {
    valid_up_to: usize,
    error_len: Option<u8>,
}

impl Utf8Error {
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    pub fn error_len(&self) -> Option<usize> {
        self.error_len.map(|len| len as usize)
    }
}

impl error::Error for Utf8Error {}

impl fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(error_len) = self.error_len {
            write!(f, "Invalid {} bytes from {}", error_len, self.valid_up_to)
        } else {
            write!(f, "Incomplete from {}", self.valid_up_to)
        }
    }
}

/// Error for UTF-8 owned string creation.
///
/// See <https://doc.rust-lang.org/stable/std/string/struct.FromUtf8Error.html>.
#[derive(Debug, Clone)]
pub struct FromUtf8Error {
    bytes: Vec<u8>,
    error: Utf8Error,
}

impl FromUtf8Error {
    fn new(error: Utf8Error, bytes: Vec<u8>) -> Self {
        Self { bytes, error }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn utf8_error(&self) -> Utf8Error {
        self.error
    }
}

impl error::Error for FromUtf8Error {}

impl fmt::Display for FromUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.error, f)
    }
}

custom_slice_macros::define_slice_types_pair! {
    /// Owned string.
    ///
    /// See <https://doc.rust-lang.org/stable/std/string/struct.String.html>.
    #[derive(Default)]
    #[custom_slice(owned)]
    #[custom_slice(derive(BorrowMut, Deref, DerefMut))]
    #[custom_slice(new_unchecked = "pub unsafe fn from_utf8_unchecked")]
    #[custom_slice(new_checked = "pub fn from_utf8")]
    #[custom_slice(error(type = "FromUtf8Error", map = "FromUtf8Error::new"))]
    pub struct StdString(Vec<u8>);

    /// String slice.
    ///
    /// See <https://doc.rust-lang.org/stable/std/primitive.str.html>.
    #[repr(transparent)]
    #[custom_slice(slice)]
    #[custom_slice(derive(DefaultRef, DefaultRefMut))]
    #[custom_slice(new_unchecked = "pub unsafe fn from_utf8_unchecked")]
    #[custom_slice(new_unchecked_mut = "pub unsafe fn from_utf8_unchecked_mut")]
    #[custom_slice(new_checked = "pub fn from_utf8")]
    #[custom_slice(new_checked_mut = "pub fn from_utf8_mut")]
    #[custom_slice(error(type = "Utf8Error", map = "{|e, _v| e}"))]
    //#[custom_slice(error(type = "Utf8Error"))]
    pub struct StdStr([u8]);

    /// Validates that the given bytes as `StdStr`.
    #[custom_slice(validator)]
    fn validate(bytes: &[u8]) -> Result<(), Utf8Error> {
        // Use `std::str::from_utf8` as validator.
        match std::str::from_utf8(bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(Utf8Error {
                valid_up_to: e.valid_up_to(),
                error_len: e.error_len().map(|len| len as u8),
            })
        }
    }
}

#[test]
fn default() {
    let _ = StdString::default();
    let _ = <&StdStr>::default();
}

#[test]
fn new_checked() {
    {
        let res: Result<StdString, FromUtf8Error> = StdString::from_utf8(b"Hello".to_vec());
        assert!(res.is_ok());
    }
    {
        let res: Result<&StdStr, Utf8Error> = StdStr::from_utf8(b"Hello");
        assert!(res.is_ok());
    }
    {
        let mut hello = b"Hello".to_vec();
        let hello_mut: &mut [u8] = &mut hello;
        let res: Result<&mut StdStr, Utf8Error> = StdStr::from_utf8_mut(hello_mut);
        assert!(res.is_ok());
    }
}

#[test]
fn new_unchecked() {
    let _: StdString = unsafe { StdString::from_utf8_unchecked(b"Hello".to_vec()) };
    let _: &StdStr = unsafe { StdStr::from_utf8_unchecked(b"Hello") };
    {
        let mut hello = b"Hello".to_vec();
        let hello_mut: &mut [u8] = &mut hello;
        let _: &mut StdStr = unsafe { StdStr::from_utf8_unchecked_mut(hello_mut) };
    }
}

#[test]
fn borrow_and_to_owned() {
    use std::borrow::{Borrow, ToOwned};

    let string = StdString::default();
    let s: &StdStr = string.borrow();
    let _: StdString = s.to_owned();
}

#[test]
fn borrow_mut() {
    use std::borrow::BorrowMut;

    let mut string = StdString::default();
    let _: &mut StdStr = <StdString as BorrowMut<StdStr>>::borrow_mut(&mut string);
}

#[test]
fn deref() {
    use std::ops::Deref;

    let string = StdString::default();
    let _: &StdStr = <StdString as Deref>::deref(&string);
}

#[test]
fn deref_mut() {
    use std::ops::DerefMut;

    let mut string = StdString::default();
    let _: &mut StdStr = <StdString as DerefMut>::deref_mut(&mut string);
}
