//! Custom string test.

// No validations.
custom_slice_macros::define_slice_types_pair! {
    #[custom_slice(owned)]
    #[derive(Default)]
    pub struct MyString(String);

    #[custom_slice(slice)]
    #[repr(transparent)]
    pub struct MyStr(str);
}

#[test]
fn create_string() {
    let _ = MyString::default();
}

//#[test]
//fn create_str() {
//    let _ = <&MyStr>::default();
//}
