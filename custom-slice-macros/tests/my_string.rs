//! Custom string test.

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

        #[test]
        fn borrow()
        where
            MyString: std::borrow::Borrow<MyStr>,
        {
        }

        #[test]
        fn default()
        where
            MyString: Default,
        {
        }

        #[test]
        fn deref()
        where
            MyString: std::ops::Deref<Target = MyStr>,
        {
        }

        #[test]
        fn deref_mut()
        where
            MyString: std::ops::DerefMut<Target = MyStr>,
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

        #[test]
        fn to_owned()
        where
            MyStr: std::borrow::ToOwned<Owned = MyString>,
        {
        }

        #[test]
        fn default_ref()
        where
            for<'a> &'a MyStr: Default,
        {
        }

        #[test]
        fn default_ref_mut()
        where
            for<'a> &'a mut MyStr: Default,
        {
        }

        #[test]
        fn default_box()
        where
            Box<MyStr>: Default,
        {
        }

        #[test]
        fn into_arc()
        where
            for<'a> std::sync::Arc<MyStr>: From<&'a MyStr>,
        {
        }

        #[test]
        fn into_box()
        where
            for<'a> Box<MyStr>: From<&'a MyStr>,
        {
        }

        #[test]
        fn into_rc()
        where
            for<'a> std::rc::Rc<MyStr>: From<&'a MyStr>,
        {
        }
    }
}
