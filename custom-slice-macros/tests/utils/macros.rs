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
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = AsRefSlice) => {
        #[test]
        fn as_ref_slice() where
            $owned: std::convert::AsRef<$slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = AsRefSliceInner) => {
        #[test]
        fn as_ref_slice_inner() where
            $owned: std::convert::AsRef<$slice_i>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = AsMutSlice) => {
        #[test]
        fn as_mut_slice() where
            $owned: std::convert::AsMut<$slice>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = AsMutSliceInner) => {
        #[test]
        fn as_mut_slice_inner() where
            $owned: std::convert::AsMut<$slice_i>,
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
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $_slice_i:ty }, target = FromInner) => {
        #[test]
        fn into_inner() where
            $owned: std::convert::From<$owned_i>,
        {}
    };
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $_slice_i:ty }, target = IntoInner) => {
        #[test]
        fn into_inner() where
            $owned_i: std::convert::From<$owned>,
        {}
    };
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $_slice_i:ty }, target = PartialEq) => {
        #[test]
        fn partial_eq() where
            $owned: std::cmp::PartialEq<$owned>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = PartialEqBulk) => {
        #[test]
        fn partial_eq_bulk() where
            $owned: std::cmp::PartialEq<$slice>,
            $slice: std::cmp::PartialEq<$owned>,
            for<'a> $owned: std::cmp::PartialEq<&'a $slice>,
            for<'a> &'a $slice: std::cmp::PartialEq<$owned>,
            for<'a> $owned: std::cmp::PartialEq<std::borrow::Cow<'a, $slice>>,
            for<'a> std::borrow::Cow<'a, $slice>: std::cmp::PartialEq<$owned>,
        {}
    };
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $slice_i:ty }, target = PartialEqInnerBulk) => {
        #[test]
        fn partial_eq_inner_bulk() where
            $owned: std::cmp::PartialEq<$slice_i>,
            $slice_i: std::cmp::PartialEq<$owned>,
            for<'a> $owned: std::cmp::PartialEq<&'a $slice_i>,
            for<'a> &'a $slice_i: std::cmp::PartialEq<$owned>,
            for<'a> $owned: std::cmp::PartialEq<std::borrow::Cow<'a, $slice_i>>,
            for<'a> std::borrow::Cow<'a, $slice_i>: std::cmp::PartialEq<$owned>,
            $owned: std::cmp::PartialEq<$owned_i>,
            $owned_i: std::cmp::PartialEq<$owned>,
        {}
    };
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $_slice_i:ty }, target = PartialOrd) => {
        #[test]
        fn partial_ord() where
            $owned: std::cmp::PartialOrd<$owned>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = PartialOrdBulk) => {
        #[test]
        fn partial_eq_bulk() where
            $owned: std::cmp::PartialOrd<$slice>,
            $slice: std::cmp::PartialOrd<$owned>,
            for<'a> $owned: std::cmp::PartialOrd<&'a $slice>,
            for<'a> &'a $slice: std::cmp::PartialOrd<$owned>,
            for<'a> $owned: std::cmp::PartialOrd<std::borrow::Cow<'a, $slice>>,
            for<'a> std::borrow::Cow<'a, $slice>: std::cmp::PartialOrd<$owned>,
        {}
    };
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $slice_i:ty }, target = PartialOrdInnerBulk) => {
        #[test]
        fn partial_ord_inner_bulk() where
            $owned: std::cmp::PartialOrd<$slice_i>,
            $slice_i: std::cmp::PartialOrd<$owned>,
            for<'a> $owned: std::cmp::PartialOrd<&'a $slice_i>,
            for<'a> &'a $slice_i: std::cmp::PartialOrd<$owned>,
            for<'a> $owned: std::cmp::PartialOrd<std::borrow::Cow<'a, $slice_i>>,
            for<'a> std::borrow::Cow<'a, $slice_i>: std::cmp::PartialOrd<$owned>,
            $owned: std::cmp::PartialOrd<$owned_i>,
            $owned_i: std::cmp::PartialOrd<$owned>,
        {}
    };
    (owned { $owned:ty: $owned_i:ty }, slice { $_slice:ty: $_slice_i:ty }, target = TryFromInner) => {
        #[test]
        fn try_from_inner() where
            $owned: std::convert::TryFrom<$owned_i>,
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
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = AsRefSlice) => {
        #[test]
        fn as_ref_slice() where
            $slice: std::convert::AsRef<$slice>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = AsRefSliceInner) => {
        #[test]
        fn as_ref_slice_inner() where
            $slice: std::convert::AsRef<$slice_i>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = AsMutSlice) => {
        #[test]
        fn as_mut_slice() where
            $slice: std::convert::AsMut<$slice>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = AsMutSliceInner) => {
        #[test]
        fn as_mut_slice_inner() where
            $slice: std::convert::AsMut<$slice_i>,
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
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = Deref) => {
        #[test]
        fn deref() where
            $slice: std::ops::Deref<Target = $slice_i>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = DerefMut) => {
        #[test]
        fn deref_mut() where
            $slice: std::ops::DerefMut<Target = $slice_i>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = FromInner) => {
        #[test]
        fn into_inner() where
            for<'a> &'a $slice: std::convert::From<&'a $slice_i>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = FromInnerMut) => {
        #[test]
        fn into_inner() where
            for<'a> &'a mut $slice: std::convert::From<&'a mut $slice_i>,
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
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = PartialEqBulk) => {
        #[test]
        fn partial_eq_bulk() where
            for<'a> $slice: std::cmp::PartialEq<&'a $slice>,
            for<'a> &'a $slice: std::cmp::PartialEq<$slice>,
            for<'a> $slice: std::cmp::PartialEq<std::borrow::Cow<'a, $slice>>,
            for<'a> std::borrow::Cow<'a, $slice>: std::cmp::PartialEq<$slice>,
        {}
    };
    (owned { $_owned:ty: $owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = PartialEqInnerBulk) => {
        #[test]
        fn partial_eq_bulk() where
            $slice: std::cmp::PartialEq<$slice_i>,
            $slice_i: std::cmp::PartialEq<$slice>,
            for<'a> $slice: std::cmp::PartialEq<&'a $slice_i>,
            for<'a> &'a $slice_i: std::cmp::PartialEq<$slice>,
            $slice: std::cmp::PartialEq<$owned_i>,
            $owned_i: std::cmp::PartialEq<$slice>,
            for<'a> $slice: std::cmp::PartialEq<std::borrow::Cow<'a, $slice_i>>,
            for<'a> std::borrow::Cow<'a, $slice_i>: std::cmp::PartialEq<$slice>,
            for<'a> &'a $slice: std::cmp::PartialEq<$slice_i>,
            for<'a> $slice_i: std::cmp::PartialEq<&'a $slice>,
            for<'a> &'a $slice: std::cmp::PartialEq<$owned_i>,
            for<'a> $owned_i: std::cmp::PartialEq<&'a $slice>,
            for<'a, 'b> &'a $slice: std::cmp::PartialEq<std::borrow::Cow<'b, $slice_i>>,
            for<'a, 'b> std::borrow::Cow<'b, $slice_i>: std::cmp::PartialEq<&'a $slice>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = PartialOrdBulk) => {
        #[test]
        fn partial_ord_bulk() where
            for<'a> $slice: std::cmp::PartialOrd<&'a $slice>,
            for<'a> &'a $slice: std::cmp::PartialOrd<$slice>,
            for<'a> $slice: std::cmp::PartialOrd<std::borrow::Cow<'a, $slice>>,
            for<'a> std::borrow::Cow<'a, $slice>: std::cmp::PartialOrd<$slice>,
        {}
    };
    (owned { $_owned:ty: $owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = PartialOrdInnerBulk) => {
        #[test]
        fn partial_ord_bulk() where
            $slice: std::cmp::PartialOrd<$slice_i>,
            $slice_i: std::cmp::PartialOrd<$slice>,
            for<'a> $slice: std::cmp::PartialOrd<&'a $slice_i>,
            for<'a> &'a $slice_i: std::cmp::PartialOrd<$slice>,
            $slice: std::cmp::PartialOrd<$owned_i>,
            $owned_i: std::cmp::PartialOrd<$slice>,
            for<'a> $slice: std::cmp::PartialOrd<std::borrow::Cow<'a, $slice_i>>,
            for<'a> std::borrow::Cow<'a, $slice_i>: std::cmp::PartialOrd<$slice>,
            for<'a> &'a $slice: std::cmp::PartialOrd<$slice_i>,
            for<'a> $slice_i: std::cmp::PartialOrd<&'a $slice>,
            for<'a> &'a $slice: std::cmp::PartialOrd<$owned_i>,
            for<'a> $owned_i: std::cmp::PartialOrd<&'a $slice>,
            for<'a, 'b> &'a $slice: std::cmp::PartialOrd<std::borrow::Cow<'b, $slice_i>>,
            for<'a, 'b> std::borrow::Cow<'b, $slice_i>: std::cmp::PartialOrd<&'a $slice>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = TryFromInner) => {
        #[test]
        fn try_from_inner() where
            for<'a> &'a $slice: std::convert::TryFrom<&'a $slice_i>,
        {}
    };
    (owned { $_owned:ty: $_owned_i:ty }, slice { $slice:ty: $slice_i:ty }, target = TryFromInnerMut) => {
        #[test]
        fn try_from_inner_mut() where
            for<'a> &'a mut $slice: std::convert::TryFrom<&'a mut $slice_i>,
        {}
    };
    (owned { $owned:ty: $_owned_i:ty }, slice { $slice:ty: $_slice_i:ty }, target = $target:ident) => {
        compile_error!("Unknown target");
    };
}
