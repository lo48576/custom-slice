//! Custom string test.

// No validations.
custom_slice_macros::define_slice_types_pair! {
    #[custom_slice(owned)]
    #[derive(Default)]
    #[custom_slice(new_unchecked = "pub fn new")]
    pub struct MyString(String);

    #[custom_slice(slice)]
    #[custom_slice(derive(Default))]
    #[repr(transparent)]
    #[custom_slice(new_unchecked = "pub fn new")]
    #[custom_slice(new_unchecked_mut = "pub fn new_mut")]
    pub struct MyStr(str);
}

#[test]
fn default() {
    let _ = MyString::default();
    let _ = <&MyStr>::default();
}

#[test]
fn new() {
    let _: MyString = unsafe { MyString::new("Hello".to_owned()) };
    let _: &MyStr = MyStr::new("Hello");
    {
        let mut hello = "Hello".to_owned();
        let hello_mut: &mut str = &mut hello;
        let _: &mut MyStr = MyStr::new_mut(hello_mut);
    }
}

#[test]
fn borrow_and_to_owned() {
    use std::borrow::{Borrow, ToOwned};

    let string = MyString::default();
    let s: &MyStr = string.borrow();
    let _: MyString = s.to_owned();
}
