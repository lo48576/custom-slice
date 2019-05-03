//! All-eq comparison test.

mod with_inner_comparison {
    use std::cmp::Ordering;

    // No validations.
    custom_slice_macros::define_slice_types_pair! {
        #[derive(Debug, PartialEq, PartialOrd)]
        #[custom_slice(owned)]
        #[custom_slice(derive(Deref, FromInner))]
        pub struct AllEqString(String);

        #[derive(Debug)]
        #[repr(transparent)]
        #[custom_slice(slice)]
        #[custom_slice(derive(FromInner))]
        pub struct AllEqStr(str);
    }

    impl PartialEq for AllEqStr {
        fn eq(&self, _: &AllEqStr) -> bool {
            true
        }
    }

    impl PartialOrd for AllEqStr {
        fn partial_cmp(&self, _: &AllEqStr) -> Option<Ordering> {
            Some(Ordering::Equal)
        }
    }

    #[test]
    fn partial_eq() {
        use std::borrow::Borrow;

        let hello_upper = AllEqString::from("HELLO".to_string());
        let hello_lower = AllEqString::from("hello".to_string());
        // Here `<String as PartialEq>` is used.
        assert_ne!(hello_upper, hello_lower);

        let hello_upper_slice: &AllEqStr = hello_upper.borrow();
        let hello_lower_slice: &AllEqStr = hello_lower.borrow();
        assert_eq!(hello_upper_slice, hello_lower_slice);
    }

    #[test]
    fn partial_cmp() {
        use std::borrow::Borrow;

        let hello_upper = AllEqString::from("HELLO".to_string());
        let hello_lower = AllEqString::from("hello".to_string());
        // Here `<String as PartialCmp>` is used.
        assert_ne!(hello_upper.partial_cmp(&hello_lower), Some(Ordering::Equal));

        let hello_upper_slice: &AllEqStr = hello_upper.borrow();
        let hello_lower_slice: &AllEqStr = hello_lower.borrow();
        assert_eq!(
            hello_upper_slice.partial_cmp(&hello_lower_slice),
            Some(Ordering::Equal)
        );
    }
}

mod with_slice_comparison {
    use std::cmp::Ordering;

    // No validations.
    custom_slice_macros::define_slice_types_pair! {
        #[derive(Debug)]
        #[custom_slice(owned)]
        #[custom_slice(derive(Deref, FromInner, PartialEq, PartialOrd))]
        pub struct AllEqString(String);

        #[derive(Debug)]
        #[repr(transparent)]
        #[custom_slice(slice)]
        #[custom_slice(derive(FromInner))]
        pub struct AllEqStr(str);
    }

    impl PartialEq for AllEqStr {
        fn eq(&self, _: &AllEqStr) -> bool {
            true
        }
    }

    impl PartialOrd for AllEqStr {
        fn partial_cmp(&self, _: &AllEqStr) -> Option<Ordering> {
            Some(Ordering::Equal)
        }
    }

    #[test]
    fn partial_eq() {
        use std::borrow::Borrow;

        let hello_upper = AllEqString::from("HELLO".to_string());
        let hello_lower = AllEqString::from("hello".to_string());
        // Here `<AllEqStr as PartialEq>` is used.
        assert_eq!(hello_upper, hello_lower);

        let hello_upper_slice: &AllEqStr = hello_upper.borrow();
        let hello_lower_slice: &AllEqStr = hello_lower.borrow();
        assert_eq!(hello_upper_slice, hello_lower_slice);
    }

    #[test]
    fn partial_cmp() {
        use std::borrow::Borrow;

        let hello_upper = AllEqString::from("HELLO".to_string());
        let hello_lower = AllEqString::from("hello".to_string());
        // Here `<AllEqStr as PartialCmp>` is used.
        assert_eq!(hello_upper.partial_cmp(&hello_lower), Some(Ordering::Equal));

        let hello_upper_slice: &AllEqStr = hello_upper.borrow();
        let hello_lower_slice: &AllEqStr = hello_lower.borrow();
        assert_eq!(
            hello_upper_slice.partial_cmp(&hello_lower_slice),
            Some(Ordering::Equal)
        );
    }
}
