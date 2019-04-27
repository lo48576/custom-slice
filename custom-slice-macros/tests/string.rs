//! Custom string test.

// No validations.
custom_slice_macros::define_slice_types_pair! {
    #[custom_slice(owned)]
    #[derive(Default)]
    pub struct MyString(String);

    #[custom_slice(slice)]
    #[custom_slice(derive(Default))]
    #[repr(transparent)]
    pub struct MyStr(str);
}

#[test]
fn default_string() {
    let _ = MyString::default();
}

#[test]
fn default_str() {
    let _ = <&MyStr>::default();
}

#[test]
fn string_conversion() {
    use std::borrow::{Borrow, ToOwned};

    let string = MyString::default();
    let s: &MyStr = string.borrow();
    let _: MyString = s.to_owned();
}
