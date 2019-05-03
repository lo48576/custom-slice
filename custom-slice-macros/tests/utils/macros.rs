//! Macros for testing.

#[allow(unused_macros)]
macro_rules! ensure_owned_traits {
    (owned { $owned:ty: $owned_inner:ty }, slice { $slice:ty: $slice_inner:ty }, traits {}) => {};
    (
        owned { $owned:ty: $owned_inner:ty },
        slice { $slice:ty: $slice_inner:ty },
        targets { $($trait:ident),* }
    ) => {
        $(
            ensure_owned_traits!(
                owned { $owned: $owned_inner },
                slice { $slice: $slice_inner },
                target = $trait
            );
        )*
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = Borrow) => {
        #[test]
        fn borrow() where
            $owned: std::borrow::Borrow<$slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = BorrowMut) => {
        #[test]
        fn borrow_mut() where
            $owned: std::borrow::BorrowMut<$slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = Default) => {
        #[test]
        fn default() where
            $owned: std::default::Default,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = Deref) => {
        #[test]
        fn deref() where
            $owned: std::ops::Deref<Target = $slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = DerefMut) => {
        #[test]
        fn deref_mut() where
            $owned: std::ops::DerefMut<Target = $slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = $target:ident) => {
        compile_error!("Unknown target");
    };
}

#[allow(unused_macros)]
macro_rules! ensure_slice_traits {
    (owned { $owned:ty: $owned_inner:ty }, slice { $slice:ty: $slice_inner:ty }, traits {}) => {};
    (
        owned { $owned:ty: $owned_inner:ty },
        slice { $slice:ty: $slice_inner:ty },
        targets { $($trait:ident),* }
    ) => {
        $(
            ensure_slice_traits!(
                owned { $owned: $owned_inner },
                slice { $slice: $slice_inner },
                target = $trait
            );
        )*
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = ToOwned) => {
        #[test]
        fn to_owned() where
            $slice: std::borrow::ToOwned<Owned = $owned>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = DefaultBox) => {
        #[test]
        fn default_box() where
            Box<$slice>: std::default::Default,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = DefaultRef) => {
        #[test]
        fn default_ref() where
            for<'a> &'a $slice: std::default::Default,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = DefaultRefMut) => {
        #[test]
        fn default_ref_mut() where
            for<'a> &'a mut $slice: std::default::Default,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = IntoArc) => {
        #[test]
        fn into_arc() where
            for<'a> std::sync::Arc<$slice>: std::convert::From<&'a $slice>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = IntoBox) => {
        #[test]
        fn into_box() where
            for<'a> std::boxed::Box<$slice>: std::convert::From<&'a $slice>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = IntoRc) => {
        #[test]
        fn into_rc() where
            for<'a> std::rc::Rc<$slice>: std::convert::From<&'a $slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = $target:ident) => {
        compile_error!("Unknown target");
    };
}
