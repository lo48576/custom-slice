//! Custom string test.

#[macro_use]
mod utils;

// No validations.
custom_slice_macros::define_slice_types_pair! {
    #[derive(Default)]
    #[custom_slice(owned)]
    #[custom_slice(derive(Deref, DerefMut))]
    #[custom_slice(new_unchecked = "
        /// Creates a new string.
        pub fn new
    ")]
    pub struct MyString(String);

    #[repr(transparent)]
    #[custom_slice(slice)]
    #[custom_slice(derive(DefaultRef, DefaultRefMut, DefaultBox, IntoArc, IntoBox, IntoRc))]
    #[custom_slice(new_unchecked = "#[allow(dead_code)] pub fn new")]
    #[custom_slice(new_unchecked_mut = "pub fn new_mut")]
    pub struct MyStr(str);
}

mod owned {
    use super::*;

    mod methods {
        use super::*;

        #[test]
        fn new() {
            let _: MyString = MyString::new("Hello".to_owned());
        }
    }

    mod traits {
        use super::*;

        ensure_owned_traits! {
            owned { MyString: String },
            slice { MyStr: str },
            targets { Borrow, Default, Deref, DerefMut }
        }
    }
}

mod slice {
    use super::*;

    mod methods {
        use super::*;

        #[test]
        fn new() {
            let _: &MyStr = MyStr::new("Hello");
        }

        #[test]
        fn new_mut() {
            let mut hello = "Hello".to_owned();
            let hello_mut: &mut str = &mut hello;
            let _: &mut MyStr = MyStr::new_mut(hello_mut);
        }
    }

    mod traits {
        use super::*;

        ensure_slice_traits! {
            owned { MyString: String },
            slice { MyStr: str },
            targets { ToOwned, DefaultRef, DefaultRefMut, DefaultBox, IntoArc, IntoBox, IntoRc }
        }
    }
}
