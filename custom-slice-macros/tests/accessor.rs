//! Custom string test.
//!
//! Note that this file may contain practically meaningless codes (such as
//! unnecessary `unsafe`).

// No validations.
custom_slice_macros::define_slice_types_pair! {
    #[custom_slice(owned)]
    #[custom_slice(new_unchecked = "pub fn new")]
    #[custom_slice(get_ref = "fn get")]
    #[custom_slice(get_mut = "unsafe fn get_mut")]
    pub struct MyString(String);

    #[repr(transparent)]
    #[custom_slice(slice)]
    #[custom_slice(new_unchecked = "pub fn new")]
    #[custom_slice(new_unchecked_mut = "pub fn new_mut")]
    #[custom_slice(get_ref = "fn get")]
    #[custom_slice(get_mut = "unsafe fn get_mut")]
    pub struct MyStr(str);
}

#[test]
fn get() {
    let owned: MyString = MyString::new("Hello".to_owned());
    {
        let _: &String = owned.get();
    }
    let slice: &MyStr = MyStr::new("Hello");
    {
        let _: &str = slice.get();
    }
}

#[test]
fn get_mut() {
    let mut owned: MyString = MyString::new("Hello".to_owned());
    {
        let _: &mut String = unsafe { owned.get_mut() };
    }
    let mut buf = "Hello".to_owned();
    let slice: &mut MyStr = MyStr::new_mut(buf.as_mut_str());
    {
        let _: &mut str = unsafe { slice.get_mut() };
    }
}
