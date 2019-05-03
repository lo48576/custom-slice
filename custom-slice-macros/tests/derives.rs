//! Simple tests.

#[macro_use]
mod utils;

macro_rules! gen_test {
    (
        name: $name:ident,
        $(#[$meta_owned:meta])* owned: $owned_inner:ty,
        owned_tests: { $($target_owned:ident),* },
        $(#[$meta_slice:meta])* slice: $slice_inner:ty,
        slice_tests: { $($target_slice:ident),* },
    ) => {
        mod $name {
            custom_slice_macros::define_slice_types_pair! {
                #[custom_slice(owned)]
                $(#[$meta_owned])*
                struct Owned($owned_inner);

                #[repr(transparent)]
                #[custom_slice(slice)]
                $(#[$meta_slice])*
                struct Slice($slice_inner);
            }

            ensure_owned_traits! {
                owned { Owned: Vec<u8> },
                slice { Slice: [u8] },
                targets { $($target_owned),* }
            }

            ensure_slice_traits! {
                owned { Owned: Vec<u8> },
                slice { Slice: [u8] },
                targets { $($target_slice),* }
            }
        }
    };
    (
        name: $name:ident,
        $(#[$meta_owned:meta])* owned: $owned_inner:ty,
        owned_tests: { $($target_owned:ident),* },
        $(#[$meta_slice:meta])* slice: $slice_inner:ty,
        slice_tests: { $($target_slice:ident),* },
        validator: $ty_error:ty $body:block,
    ) => {
        mod $name {
            custom_slice_macros::define_slice_types_pair! {
                #[custom_slice(owned)]
                $(#[$meta_owned])*
                struct Owned($owned_inner);

                #[repr(transparent)]
                #[custom_slice(slice)]
                $(#[$meta_slice])*
                struct Slice($slice_inner);

                #[custom_slice(validator)]
                fn validate(_: &$slice_inner) -> Result<(), $ty_error> $body
            }

            ensure_owned_traits! {
                owned { Owned: Vec<u8> },
                slice { Slice: [u8] },
                targets { $($target_owned),* }
            }

            ensure_slice_traits! {
                owned { Owned: Vec<u8> },
                slice { Slice: [u8] },
                targets { $($target_slice),* }
            }
        }
    };
}

mod owned {
    gen_test! {
        name: borrow,
        owned: Vec<u8>,
        owned_tests: { Borrow },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: as_ref_slice,
        #[custom_slice(derive(AsRefSlice))]
        owned: Vec<u8>,
        owned_tests: { AsRefSlice },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: as_ref_slice_inner,
        #[custom_slice(derive(AsRefSliceInner))]
        owned: Vec<u8>,
        owned_tests: { AsRefSliceInner },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: as_mut_slice,
        #[custom_slice(derive(AsMutSlice))]
        owned: Vec<u8>,
        owned_tests: { AsMutSlice },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: as_mut_slice_inner,
        #[custom_slice(derive(AsMutSliceInner))]
        owned: Vec<u8>,
        owned_tests: { AsMutSliceInner },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: borrow_mut,
        #[custom_slice(derive(BorrowMut))]
        owned: Vec<u8>,
        owned_tests: { BorrowMut },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: deref,
        #[custom_slice(derive(Deref))]
        owned: Vec<u8>,
        owned_tests: { Deref },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: deref_mut,
        #[custom_slice(derive(Deref, DerefMut))]
        owned: Vec<u8>,
        owned_tests: { Deref, DerefMut },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: from_inner,
        #[custom_slice(derive(FromInner))]
        owned: Vec<u8>,
        owned_tests: { FromInner },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: into_inner,
        #[custom_slice(derive(IntoInner))]
        owned: Vec<u8>,
        owned_tests: { IntoInner },
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: partial_eq,
        #[custom_slice(derive(PartialEq))]
        owned: Vec<u8>,
        owned_tests: { PartialEq },
        #[derive(PartialEq)]
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: partial_eq_bulk,
        #[derive(PartialEq)]
        #[custom_slice(derive(PartialEqBulk))]
        owned: Vec<u8>,
        owned_tests: { PartialEqBulk },
        #[derive(PartialEq)]
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: partial_ord,
        #[custom_slice(derive(PartialEq, PartialOrd))]
        owned: Vec<u8>,
        owned_tests: { PartialOrd },
        #[derive(PartialEq, PartialOrd)]
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: partial_ord_bulk,
        #[derive(PartialEq, PartialOrd)]
        #[custom_slice(derive(PartialEqBulk, PartialOrdBulk))]
        owned: Vec<u8>,
        owned_tests: { PartialOrdBulk },
        #[derive(PartialEq, PartialOrd)]
        slice: [u8],
        slice_tests: {},
    }

    gen_test! {
        name: try_from_inner,
        #[custom_slice(derive(TryFromInner))]
        #[custom_slice(error(r#type = "()"))]
        owned: Vec<u8>,
        owned_tests: { TryFromInner },
        #[custom_slice(error(r#type = "()"))]
        slice: [u8],
        slice_tests: {},
        validator: () { Ok(()) },
    }
}

mod slice {
    gen_test! {
        name: to_owned,
        owned: Vec<u8>,
        owned_tests: {},
        slice: [u8],
        slice_tests: { ToOwned },
    }

    gen_test! {
        name: as_ref_slice,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(AsRefSlice))]
        slice: [u8],
        slice_tests: { AsRefSlice },
    }

    gen_test! {
        name: as_ref_slice_inner,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(AsRefSliceInner))]
        slice: [u8],
        slice_tests: { AsRefSliceInner },
    }

    gen_test! {
        name: as_mut_slice,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(AsMutSlice))]
        slice: [u8],
        slice_tests: { AsMutSlice },
    }

    gen_test! {
        name: as_mut_slice_inner,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(AsMutSliceInner))]
        slice: [u8],
        slice_tests: { AsMutSliceInner },
    }

    gen_test! {
        name: default_box,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(DefaultBox))]
        slice: [u8],
        slice_tests: { DefaultBox },
    }

    gen_test! {
        name: default_ref,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(DefaultRef))]
        slice: [u8],
        slice_tests: { DefaultRef },
    }

    gen_test! {
        name: default_ref_mut,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(DefaultRefMut))]
        slice: [u8],
        slice_tests: { DefaultRefMut },
    }

    gen_test! {
        name: from_inner,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(FromInner))]
        slice: [u8],
        slice_tests: { FromInner },
    }

    gen_test! {
        name: from_inner_mut,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(FromInnerMut))]
        slice: [u8],
        slice_tests: { FromInnerMut },
    }

    gen_test! {
        name: into_arc,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(IntoArc))]
        slice: [u8],
        slice_tests: { IntoArc },
    }

    gen_test! {
        name: into_box,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(IntoBox))]
        slice: [u8],
        slice_tests: { IntoBox },
    }

    gen_test! {
        name: into_rc,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(IntoRc))]
        slice: [u8],
        slice_tests: { IntoRc },
    }

    gen_test! {
        name: partial_eq_bulk,
        owned: Vec<u8>,
        owned_tests: {},
        #[derive(PartialEq)]
        #[custom_slice(derive(PartialEqBulk))]
        slice: [u8],
        slice_tests: { PartialEqBulk },
    }

    gen_test! {
        name: partial_eq_inner_bulk,
        owned: Vec<u8>,
        owned_tests: {},
        #[derive(PartialEq)]
        #[custom_slice(derive(PartialEqInnerBulk))]
        slice: [u8],
        slice_tests: { PartialEqInnerBulk },
    }

    gen_test! {
        name: partial_ord_bulk,
        owned: Vec<u8>,
        owned_tests: {},
        #[derive(PartialEq, PartialOrd)]
        #[custom_slice(derive(PartialEqBulk, PartialOrdBulk))]
        slice: [u8],
        slice_tests: { PartialOrdBulk },
    }

    gen_test! {
        name: partial_ord_inner_bulk,
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(PartialEqInnerBulk, PartialOrdInnerBulk))]
        slice: [u8],
        slice_tests: { PartialOrdInnerBulk },
    }

    gen_test! {
        name: try_from_inner,
        #[custom_slice(error(r#type = "()"))]
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(TryFromInner))]
        #[custom_slice(error(r#type = "()"))]
        slice: [u8],
        slice_tests: { TryFromInner },
        validator: () { Ok(()) },
    }

    gen_test! {
        name: try_from_inner_mut,
        #[custom_slice(error(r#type = "()"))]
        owned: Vec<u8>,
        owned_tests: {},
        #[custom_slice(derive(TryFromInnerMut))]
        #[custom_slice(error(r#type = "()"))]
        slice: [u8],
        slice_tests: { TryFromInnerMut },
        validator: () { Ok(()) },
    }
}
