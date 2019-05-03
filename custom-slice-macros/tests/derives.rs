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
}
