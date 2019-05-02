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

#[test]
fn default() {
    let _ = MyString::default();
    let _ = <&MyStr>::default();
}

#[test]
fn new() {
    let _: MyString = MyString::new("Hello".to_owned());
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

#[test]
fn deref() {
    use std::ops::Deref;

    let string = MyString::default();
    let _: &MyStr = <MyString as Deref>::deref(&string);
}

#[test]
fn deref_mut() {
    use std::ops::DerefMut;

    let mut string = MyString::default();
    let _: &mut MyStr = <MyString as DerefMut>::deref_mut(&mut string);
}

#[test]
fn default_box() {
    let _: Box<MyStr> = Default::default();
}

#[test]
fn into_arc() {
    use std::sync::Arc;

    let s: &MyStr = Default::default();
    let _: Arc<MyStr> = Arc::<MyStr>::from(s);
}

#[test]
fn into_box() {
    let s: &MyStr = Default::default();
    let _: Box<MyStr> = Box::<MyStr>::from(s);
}

#[test]
fn into_rc() {
    use std::rc::Rc;

    let s: &MyStr = Default::default();
    let _: Rc<MyStr> = Rc::<MyStr>::from(s);
}
