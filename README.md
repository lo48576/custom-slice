# custom-slice

[![Build Status](https://travis-ci.org/lo48576/custom-slice.svg?branch=develop)](https://travis-ci.org/lo48576/custom-slice)
![Minimum rustc version: 1.34](https://img.shields.io/badge/rustc-1.34+-lightgray.svg)

`custom-slice-macros`:
[![Latest version](https://img.shields.io/crates/v/custom-slice-macros.svg)](https://crates.io/crates/custom-slice-macros)
[![Documentation](https://docs.rs/custom-slice-macros/badge.svg)](https://docs.rs/custom-slice-macros)

Proc-macros to define custom slice types easily (without users writing unsafe
codes manually).

## Usage

Consider the case you want to define slice types as below:

```rust
/// Owned slice.
// Sized type.
pub struct Owned(OwnedInner);

/// Borrowed slice.
// Unsized slice type.
pub struct Slice(SliceInner);

impl std::borrow::Borrow<Slice> for Owned { /* .. */ }

impl std::borrow::ToOwned for Slice {
    type Owned = Owned;

    // ..
}
```

For example, if `Owned` is `String` and `Slice` is `str`, `OwnedInner` is
`Vec<u8>` and `SliceInner` is `[u8]`.

### Basic

```rust
custom_slice_macros::define_slice_types_pair! {
    /// Owned slice.
    #[custom_slice(owned)]
    pub struct Owned(OwnedInner);

    /// Borrowed slice.
    #[repr(transparent)]
    #[custom_slice(slice)]
    pub struct Slice(SliceInner);
}
```

By this way, `std::borrow::Borrow` and `std::borrow::ToOwned` is automatically
implemented.

Note that:

* `#[repr(transparent)]` or `#[repr(C)]` is required for slice type.
* Any attributes for the types will be emitted if it is not
  `#[custom_slice(..)]` style.
    + You can specify `#[derive(Debug, Clone, Copy, ..)]` for the types.
* Visibility will be not modified.
    + Instead of `pub`, you can use any valid visibility
      (such as `pub(crate)` or nothing).

### Constructor, error and validator

You can specify validator functions and error types for constructors.

This is useful for types which can have limited values (compared to the inner
types).
For example, `Vec<u8>` can have any binary data, but `String` can have valid
UTF-8 sequences.

* Specify constructor names, visilibily, and unsafety.
  All attributes below are optional.
    + `#[custom_slice(new_unchecked = ..)]`: constructor without validation.
        * This does NOT require validator.
        * This returns `Owned`.
    + `#[custom_slice(new_checked = ..)]`: constructor with validation.
        * This requires validator.
        * This returns `Result<Owned, _>`.
    + `#[custom_slice(new_unchecked_mut = ..)]`: constructor without validation.
        * This does NOT require validator.
        * Available only for slice types.
        * This returns `&mut Slice`.
    + `#[custom_slice(new_checked_mut = ..)]`: constructor with validation.
        * This requires validator.
        * Available only for slice types.
        * This returns `Result<&mut Slice, _>`.
* Specify validator function.
    + Optional.
    + Validator function name can have any valid name, but should be defined in
      the `define_slice_types_pair!` macro and should have
      `#[custom_slice(validator)]` attribute.
    + Return type should be `std::result::Result<(), _>`.
* Specify Error type and mapping function.
    + Use `#[custom_slice(error(type = "ErrorTypeName"))]`.
    + If you want to return modified error, use
      `#[custom_slice(error(type = "ErrorTypeName", map = "mapping_expr"))]`.
    + `type` is mandatory if you specify `new_checked` or `new_checked_mut`,
      but `map` is optional in such cases.
    + `mapping_expr` can be any function expression with type
      `FnOnce(ValidatorError, Inner) -> CtorError`.

Example without validator:

```rust
custom_slice_macros::define_slice_types_pair! {
    /// Owned slice.
    // Assume `owned_inner: OwnedInner`.
    #[custom_slice(owned)]
    //let _: Owned = Owned::new(owned_inner);
    #[custom_slice(new_unchecked = "fn new")]
    pub struct Owned(OwnedInner);

    /// Borrowed slice.
    #[repr(transparent)]
    // Assume `slice_inner_ref: &Slice` and `slice_inner_mut: &mut Slice`.
    #[custom_slice(slice)]
    //let _: &Slice = Slice::new(slice_inner_ref);
    #[custom_slice(new = "fn new")]
    //let _: &mut Slice = Slice::new_mut(slice_inner_mut);
    #[custom_slice(new_mut = "fn new_mut")]
    pub struct Slice(SliceInner);
}
```

Example with validator:

```rust
custom_slice_macros::define_slice_types_pair! {
    /// Owned slice.
    // Assume `owned_inner: OwnedInner`.
    #[custom_slice(owned)]
    //let _: Owned = unsafe { Owned::new_unchecked(owned_inner) };
    #[custom_slice(new_unchecked = "unsafe fn new_unchecked")]
    //let _: Result<Owned, ErrorWithInner> = Owned::new(owned_inner);
    #[custom_slice(new_checked = "pub fn new")]
    #[custom_slice(error(
        type = "ErrorWithInner",
        map = "{|e, v| Error { error: e, value: v } }"
    ))]
    pub struct Owned(OwnedInner);

    /// Borrowed slice.
    #[repr(transparent)]
    // Assume `slice_inner_ref: &Slice` and `slice_inner_mut: &mut Slice`.
    #[custom_slice(slice)]
    //let _: &Slice = unsafe { Slice::new_unchecked(slice_inner_ref) };
    #[custom_slice(new_unchecked = "unsafe fn new_unchecked")]
    //let _: &mut Slice = unsafe { Slice::new_unchecked_mut(slice_inner_mut) };
    #[custom_slice(new_unchecked_mut = "unsafe fn new_unchecked_mut")]
    //let _: Result<&Slice, Error> = Slice::new(slice_inner_ref);
    #[custom_slice(new_checked = "pub fn new")]
    //let _: Result<&mut Slice, Error> = Slice::new_mut(slice_inner_mut);
    #[custom_slice(new_checked_mut = "pub fn new_mut")]
    #[custom_slice(error(type = "Error"))]
    pub struct Slice(SliceInner);

    /// Validates the given data.
    ///
    /// Returns `Ok(())` for valid data, `Err(_)` for invalid data.
    #[custom_slice(validator)]
    fn validate(s: &SliceInner) -> Result<(), Error> {
        /* Do the validation. */
    }
}
```

### Accessors

You can define accessors to the inner types with meaningful name.

```rust
custom_slice_macros::define_slice_types_pair! {
    /// Owned slice.
    // Assume `owned: Owned` and `mut owned_mut: Owned`.
    #[custom_slice(owned)]
    //let _: &OwnedInner = owned.get();
    #[custom_slice(get_ref = "pub fn get")]
    //let _: &mut OwnedInner = owned_mut.get_mut();
    #[custom_slice(get_mut = "fn get_mut")]
    //let _: OwnedInner = owned.into_inner();
    #[custom_slice(into_inner = "pub fn into_inner")]
    pub struct Owned(OwnedInner);

    /// Borrowed slice.
    // Assume `slice_ref: &Slice` and `slice_mut: &mut Slice`.
    #[repr(transparent)]
    #[custom_slice(slice)]
    //let _: &SliceInner = slice_ref.get();
    #[custom_slice(get_ref = "pub fn get")]
    //let _: &mut SliceInner = slice_mut.get_mut();
    #[custom_slice(get_mut = "fn get_mut")]
    pub struct Slice(SliceInner);
}
```

* Specify accessor names, visilibily, and unsafety.
  All attributes below are optional.
    + `#[custom_slice(get_ref = ..)]`: reference getter.
        * This returns `&OwnedInner` or `&SliceInner`.
    + `#[custom_slice(get_mut = ..)]`: mutable reference getter.
        * This returns `&mut OwnedInner` or `&mut SliceInner`.
    + `#[custom_slice(into_inner = ..)]`: deconstructor.
        * This returns `OwnedInner`.
        * This is available only for owned types.

### Comments and attributes for functions
In attributes to specify functions (such as `get_ref` and `new_unchecked`), you
can specify attributes and comments.

For example:

```rust
custom_slice_macros::define_slice_types_pair! {
    /// Owned slice.
    #[custom_slice(owned)]
    #[custom_slice(get_ref = "#[allow(missing_docs)] pub fn get")]
    #[custom_slice(get_mut = "#[deny(dead_code)] fn get_mut")]
    #[custom_slice(into_inner = "
        /// Extracts the inner owned slice.
        pub fn into_inner
    ")]
    pub struct Owned(OwnedInner);

    /// Borrowed slice.
    #[repr(transparent)]
    #[custom_slice(slice)]
    #[custom_slice(new_unchecked = "
        /// Creates a new `Slice` without validation.
        #[deprecated (since = \"0.2.0\", note = \"Use `new_checked`\")]
        pub fn new_unchecked
    ")]
    #[custom_slice(new_checked = "
        /// Creates a new `Slice` if the given value is valid.
        pub fn new_checked
    ")]
    pub struct Slice(SliceInner);
}
```

### Deriving traits

`custom_slice_macros::define_slice_types_pair!` supports generating impls which
should possibly require unsafe operations.

```rust
custom_slice_macros::define_slice_types_pair! {
    /// Owned slice.
    #[custom_slice(owned)]
    #[custom_slice(derive(BorrowMut, Deref, DerefMut))]
    pub struct Owned(OwnedInner);

    /// Borrowed slice.
    #[repr(transparent)]
    #[custom_slice(slice)]
    #[custom_slice(derive(DefaultRef, DefaultRefMut))]
    pub struct Slice(SliceInner);
}
```

The following derive targets are available:

* For owned types:
    + `AsRefSlice`:
      `impl std::convert::AsRef<Slice> for Owned { /* .. */ }`
        * Requires `AsRef<SliceInner>: OwnedInner`.
    + `AsRefSliceInner`:
      `impl std::convert::AsRef<SliceInner> for Owned { /* .. */ }`
        * Requires `AsRef<SliceInner>: OwnedInner`.
    + `AsMutSlice`:
      `impl std::convert::AsMut<Slice> for Owned { /* .. */ }`
        * Requires `AsMut<SliceInner>: OwnedInner`.
    + `AsMutSliceInner`:
      `impl std::convert::AsMut<SliceInner> for Owned { /* .. */ }`
        * Requires `AsMut<SliceInner>: OwnedInner`.
    + `BorrowMut`:
      `impl std::borrow::BorrowMut<Slice> for Owned { /* .. */ }`
    + `Deref`:
      `impl std::ops::Deref for Owned { type Target = Slice; /* .. */ }`
    + `DerefMut`:
      `impl std::ops::DerefMut for Owned { /* .. */ }`
    + `FromInner`:
      `impl std::convert::From<OwnedInner> for Owned { /* .. */ }`
        * Requires validator to be absent.
    + `IntoInner`:
      `impl std::convert::From<Owned> for OwnedInner { /* .. */ }`
    + `TryFromInner`:
      `impl std::convert::TryFrom<OwnedInner> for Owned { /* .. */ }`
        * Requires validator to be present.
* For slice types:
    + `AsRefSlice`:
      `impl std::convert::AsRef<Slice> for Slice { /* .. */ }`
        * Requires `AsRef<SliceInner>: SliceInner`.
    + `AsRefSliceInner`:
      `impl std::convert::AsRef<SliceInner> for Slice { /* .. */ }`
        * Requires `AsRef<SliceInner>: SliceInner`.
    + `AsMutSlice`:
      `impl std::convert::AsMut<Slice> for Slice { /* .. */ }`
        * Requires `AsMut<SliceInner>: SliceInner`.
    + `AsMutSliceInner`:
      `impl std::convert::AsMut<SliceInner> for Slice { /* .. */ }`
        * Requires `AsMut<SliceInner>: SliceInner`.
    + `DefaultBox`:
      `impl std::default::Default for Box<Slice> { /* .. */ }`
        * Requires `Box<SliceInner>: Default`.
    + `DefaultRef`:
      `impl std::default::Default for &Slice { /* .. */ }`
        * Requires `&SliceInner: Default`.
    + `DefaultRefMut`:
      `impl std::default::Default for &mut Slice { /* .. */ }`
        * Requires `&mut SliceInner: Default`.
    + `FromInner`:
      `impl<'a> std::convert::From<&'a SliceInner> for &'a Slice { /* .. */ }`
        * Requires validator to be absent.
    + `FromInnerMut`:
      `impl<'a> std::convert::From<&'a mut SliceInner> for &'a mut Slice { /* .. */ }`
        * Requires validator to be absent.
    + `IntoArc`:
      `impl std::convert::From<&Slice> for std::sync::Arc<Slice> { /* .. */ }`
        * Requires `Arc<SliceInner>: From<&SliceInner>`.
    + `IntoBox`:
      `impl std::convert::From<&Slice> for std::boxed::Box<Slice> { /* .. */ }`
        * Requires `Box<SliceInner>: From<&SliceInner>`.
    + `IntoRc`:
      `impl std::convert::From<&Slice> for std::rc::Rc<Slice> { /* .. */ }`
        * Requires `Rc<SliceInner>: From<&SliceInner>`.
    + `TryFromInner`:
      `impl<'a> std::convert::TryFrom<&'a SliceInner> for &'a Slice { /* .. */ }`
        * Requires validator to be present.
    + `TryFromInnerMut`:
      `impl<'a> std::convert::TryFrom<&'a mut SliceInner> for &'a mut Slice { /* .. */ }`
        * Requires validator to be present.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE.txt](LICENSE-APACHE.txt) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT.txt](LICENSE-MIT.txt) or
  <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
