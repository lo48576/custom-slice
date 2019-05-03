//! Custom string test.

use std::{error, fmt};

/// Error for lower ascii string creation.
#[derive(Debug, Clone, Copy)]
pub struct Error(char);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Non-ascii-lowercase character: {:?}", self.0)
    }
}

impl error::Error for Error {}

custom_slice_macros::define_slice_types_pair! {
    /// A string which contains only lower ascii characters.
    #[derive(Default)]
    #[custom_slice(owned)]
    #[custom_slice(derive(Deref, DerefMut))]
    #[custom_slice(new_unchecked = "unsafe fn new_unchecked")]
    #[custom_slice(new_checked = "pub fn new")]
    #[custom_slice(error(type = "Error"))]
    pub struct LowerAsciiString(String);

    /// A string which contains only lower ascii characters.
    #[repr(transparent)]
    #[custom_slice(slice)]
    #[custom_slice(derive(DefaultRef, DefaultRefMut))]
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

mod owned {
    use super::*;

    mod methods {
        use super::*;

        #[test]
        fn new() {
            let res: Result<LowerAsciiString, Error> = LowerAsciiString::new("hello".to_owned());
            assert!(res.is_ok());
        }

        #[test]
        fn new_should_fail() {
            assert!(LowerAsciiString::new("Hello".to_owned()).is_err());
        }

        #[test]
        fn new_unchecked() {
            let _: LowerAsciiString =
                unsafe { LowerAsciiString::new_unchecked("hello".to_owned()) };
        }
    }

    mod traits {
        use super::*;

        #[test]
        fn borrow()
        where
            LowerAsciiString: std::borrow::Borrow<LowerAsciiStr>,
        {
        }

        #[test]
        fn default()
        where
            LowerAsciiString: Default,
        {
        }

        #[test]
        fn deref()
        where
            LowerAsciiString: std::ops::Deref<Target = LowerAsciiStr>,
        {
        }

        #[test]
        fn deref_mut()
        where
            LowerAsciiString: std::ops::DerefMut<Target = LowerAsciiStr>,
        {
        }
    }
}

mod slice {
    use super::*;

    mod methods {
        use super::*;

        #[test]
        fn new() {
            let res: Result<&LowerAsciiStr, Error> = LowerAsciiStr::new("hello");
            assert!(res.is_ok());
        }

        #[test]
        fn new_should_fail() {
            assert!(LowerAsciiStr::new("Hello").is_err());
        }

        #[test]
        fn new_mut() {
            let mut hello = "hello".to_owned();
            let hello_mut: &mut str = &mut hello;
            let res: Result<&mut LowerAsciiStr, Error> = LowerAsciiStr::new_mut(hello_mut);
            assert!(res.is_ok());
        }

        #[test]
        fn new_mut_should_fail() {
            let mut hello = "Hello".to_owned();
            let hello_mut: &mut str = &mut hello;
            assert!(LowerAsciiStr::new(hello_mut).is_err());
        }

        #[test]
        fn new_unchecked() {
            let _: &LowerAsciiStr = unsafe { LowerAsciiStr::new_unchecked("hello") };
            {
                let mut hello = "hello".to_owned();
                let hello_mut: &mut str = &mut hello;
                let _: &mut LowerAsciiStr = unsafe { LowerAsciiStr::new_unchecked_mut(hello_mut) };
            }
        }
    }

    mod traits {
        use super::*;

        #[test]
        fn to_owned()
        where
            LowerAsciiStr: std::borrow::ToOwned<Owned = LowerAsciiString>,
        {
        }

        #[test]
        fn default_ref()
        where
            for<'a> &'a LowerAsciiStr: Default,
        {
        }
    }
}
