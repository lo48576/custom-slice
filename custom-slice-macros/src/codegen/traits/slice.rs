//! Trait impls for slice types.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    codegen::{
        expr::{Owned, Slice, SliceInner},
        props::{Constant, DynMutability, Mutability, Safety},
        traits::{CmpTrait, OwnedToSliceTrait},
        types::{RefType, SmartPtr, SmartPtrExt},
    },
    defs::Definitions,
};

/// Implements `AsRef<Slice>` or `AsMut<Slice>`.
pub(crate) fn impl_as_ref_slice(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let trait_as_ref = OwnedToSliceTrait::AsRef.trait_path(mutability);
    let fn_as_ref = OwnedToSliceTrait::AsRef.method_name(mutability);

    let ty_slice = defs.ty_slice();
    let ty_slice_ref = mutability.make_ref(&ty_slice);
    let self_ref = mutability.make_ref(quote!(self));
    quote! {
        impl #trait_as_ref<#ty_slice> for #ty_slice {
            fn #fn_as_ref(#self_ref) -> #ty_slice_ref {
                self
            }
        }
    }
}

/// Implements `AsRef<SliceInner>` or `AsMut<SliceInner>`.
pub(crate) fn impl_as_ref_slice_inner(
    defs: &Definitions,
    mutability: impl Mutability,
) -> TokenStream {
    let trait_as_ref = OwnedToSliceTrait::AsRef.trait_path(mutability);
    let fn_as_ref = OwnedToSliceTrait::AsRef.method_name(mutability);

    let ty_slice = defs.ty_slice();
    let ty_slice_inner = defs.ty_slice_inner();
    let ty_slice_inner_ref = mutability.make_ref(&ty_slice_inner);
    let self_ref = mutability.make_ref(quote!(self));

    let body: SliceInner<_, _> = Slice::new(quote!(self), mutability).to_slice_inner_ref(defs);
    quote! {
        impl #trait_as_ref<#ty_slice_inner> for #ty_slice {
            fn #fn_as_ref(#self_ref) -> #ty_slice_inner_ref {
                #body
            }
        }
    }
}

/// Implements `PartialEq` and `PartialOrd` for many types.
pub(crate) fn impl_cmp_bulk(defs: &Definitions, target: CmpTrait) -> TokenStream {
    let mut tokens = TokenStream::new();
    target
        .impl_with_slice(defs, RefType::Slice, RefType::RefSlice)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::RefSlice, RefType::Slice)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::Slice, RefType::CowSlice)
        .to_tokens(&mut tokens);
    target
        .impl_with_slice(defs, RefType::CowSlice, RefType::Slice)
        .to_tokens(&mut tokens);

    tokens
}

/// Implements `PartialEq` and `PartialOrd` for many types.
pub(crate) fn impl_cmp_inner_bulk(defs: &Definitions, target: CmpTrait) -> TokenStream {
    let mut tokens = TokenStream::new();
    target
        .impl_with_inner(defs, RefType::Slice, RefType::SliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::SliceInner, RefType::Slice)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::Slice, RefType::RefSliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::RefSliceInner, RefType::Slice)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::Slice, RefType::OwnedInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::OwnedInner, RefType::Slice)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::Slice, RefType::CowSliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::CowSliceInner, RefType::Slice)
        .to_tokens(&mut tokens);

    target
        .impl_with_inner(defs, RefType::RefSlice, RefType::SliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::SliceInner, RefType::RefSlice)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::RefSlice, RefType::OwnedInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::OwnedInner, RefType::RefSlice)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::RefSlice, RefType::CowSliceInner)
        .to_tokens(&mut tokens);
    target
        .impl_with_inner(defs, RefType::CowSliceInner, RefType::RefSlice)
        .to_tokens(&mut tokens);

    tokens
}

/// Implements `Default` for `&Slice` or `&mut Slice`.
pub(crate) fn impl_default_ref(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let ty_slice_ref = mutability.make_ref(defs.ty_slice());
    let ty_slice_inner_ref = mutability.make_ref(defs.ty_slice_inner());

    let body: Slice<_, _> = SliceInner::new(
        quote!(<#ty_slice_inner_ref as std::default::Default>::default()),
        mutability,
    )
    .to_slice_unchecked(defs, Safety::Safe);
    quote! {
        impl std::default::Default for #ty_slice_ref {
            fn default() -> Self {
                #body
            }
        }
    }
}

/// Implements `Default` for `{Arc, Box, Rc}<Slice>`.
pub(crate) fn impl_default_smartptr(defs: &Definitions, smartptr: impl SmartPtr) -> TokenStream {
    let ty_slice = defs.ty_slice();
    let ty_slice_inner = defs.ty_slice_inner();

    let ty_smartptr_slice = smartptr.ty(&ty_slice);
    let expr_from_raw = {
        let default_smartptr_inner = {
            let ty_smartptr_slice_inner = smartptr.ty(&ty_slice_inner);
            quote!(<#ty_smartptr_slice_inner as std::default::Default>::default())
        };
        let expr_into_raw_inner = smartptr.expr_into_raw(ty_slice_inner, default_smartptr_inner);
        smartptr.expr_from_raw(&ty_slice, quote!(#expr_into_raw_inner as *mut #ty_slice))
    };
    quote! {
        impl std::default::Default for #ty_smartptr_slice {
            fn default() -> Self {
                unsafe { #expr_from_raw }
            }
        }
    }
}

/// Implements `Deref` or `DerefMut`.
pub(crate) fn impl_deref(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let trait_deref = OwnedToSliceTrait::Deref.trait_path(mutability);
    let fn_deref = OwnedToSliceTrait::Deref.method_name(mutability);

    let ty_slice = defs.ty_slice();
    let self_ref = mutability.make_ref(quote!(self));
    let target = match mutability.into() {
        DynMutability::Constant => {
            let ty_slice_inner = defs.ty_slice_inner();
            quote!(type Target = #ty_slice_inner;)
        }
        DynMutability::Mutable => quote!(),
    };
    let ty_ret = mutability.make_ref(quote!(Self::Target));

    let body: SliceInner<_, _> = Slice::new(quote!(self), mutability).to_slice_inner_ref(defs);
    quote! {
        impl #trait_deref for #ty_slice {
            #target

            fn #fn_deref(#self_ref) -> #ty_ret {
                #body
            }
        }
    }
}

/// Implements `TryFrom<SliceInner>`.
pub(crate) fn impl_from_inner(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    if defs.has_validator() {
        panic!("`From<SliceInner>` cannot be implemented because a validator is specified");
    }

    let lt = quote!('a);
    let ty_slice_ref = mutability.make_ref_with_lifetime(defs.ty_slice(), &lt);
    let ty_slice_inner_ref = mutability.make_ref_with_lifetime(defs.ty_slice_inner(), &lt);
    let arg_name = SliceInner::new(quote!(_v), mutability);
    let body = arg_name.to_slice_unchecked(defs, Safety::Safe);
    quote! {
        impl<#lt> std::convert::From<#ty_slice_inner_ref> for #ty_slice_ref {
            fn from(#arg_name: #ty_slice_inner_ref) -> Self {
                #body
            }
        }
    }
}

/// Implements `From<&Slice>` for `{Arc, Box, Rc}<Slice>`.
pub(crate) fn impl_into_smartptr(defs: &Definitions, smartptr: impl SmartPtr) -> TokenStream {
    let ty_slice = defs.ty_slice();
    let ty_slice_inner = defs.ty_slice_inner();
    let arg_name = Slice::new(quote!(_v), Constant);

    let ty_smartptr_slice = smartptr.ty(&ty_slice);
    let expr_from_raw = {
        let expr_smartptr_inner = {
            let ty_smartptr_slice_inner = smartptr.ty(&ty_slice_inner);
            let arg_inner_ref: SliceInner<_, _> = arg_name.to_slice_inner_ref(defs);
            quote!(<#ty_smartptr_slice_inner>::from(#arg_inner_ref))
        };
        let expr_into_raw_inner = smartptr.expr_into_raw(ty_slice_inner, expr_smartptr_inner);
        smartptr.expr_from_raw(&ty_slice, quote!(#expr_into_raw_inner as *mut #ty_slice))
    };
    quote! {
        impl std::convert::From<&#ty_slice> for #ty_smartptr_slice {
            fn from(#arg_name: &#ty_slice) -> Self {
                unsafe { #expr_from_raw }
            }
        }
    }
}

/// Implements `ToOwned`.
pub(crate) fn impl_to_owned(defs: &Definitions) -> TokenStream {
    let ty_owned = defs.ty_owned();
    let ty_slice = defs.ty_slice();

    // `&Slice` -> `&SliceInner` -> `OwnedInner` -> `Owned`.
    let body: Owned<_> = Slice::new(quote!(self), Constant)
        .to_slice_inner_ref(defs)
        .to_owned_inner(defs)
        .to_owned_unchecked(defs);
    quote! {
        impl std::borrow::ToOwned for #ty_slice {
            type Owned = #ty_owned;

            fn to_owned(&self) -> Self::Owned {
                #body
            }
        }
    }
}

/// Implements `TryFrom<SliceInner>`.
pub(crate) fn impl_try_from_inner(defs: &Definitions, mutability: impl Mutability) -> TokenStream {
    let arg_name = SliceInner::new(quote!(_v), mutability);
    let error_var = &quote!(_e);
    let lt = quote!('a);

    let ty_slice_ref = mutability.make_ref_with_lifetime(defs.ty_slice(), &lt);
    let ty_slice_inner_ref = mutability.make_ref_with_lifetime(defs.ty_slice_inner(), &lt);

    let (body, ty_error) = inner_to_outer_checked(defs, arg_name.as_ref(), error_var, Safety::Safe);
    quote! {
        impl<#lt> std::convert::TryFrom<#ty_slice_inner_ref> for #ty_slice_ref {
            type Error = #ty_error;

            fn try_from(#arg_name: #ty_slice_inner_ref) -> std::result::Result<Self, Self::Error> {
                #body
            }
        }
    }
}

/// Returns `(expr_result_outer, ty_error)`.
pub(crate) fn inner_to_outer_checked(
    defs: &Definitions,
    inner_var: SliceInner<impl ToTokens, impl Mutability>,
    error_var: impl ToTokens,
    safety: Safety,
) -> (TokenStream, syn::Type) {
    let (ty_error, mapped_error) = defs.slice_error_ty_and_val(&error_var, inner_var.as_ref());

    let expr_slice = inner_var.to_slice_unchecked(defs, safety);
    let fn_validate = defs.fn_validator().unwrap_or_else(|| {
        panic!("Validator should be necessary for `TryFromInner` derive target")
    });
    let expr = quote! {
        match #fn_validate(#inner_var) {
            Ok(_) => Ok(#expr_slice),
            Err(#error_var) => Err(#mapped_error),
        }
    };
    (expr, ty_error)
}
