//! Proc macros to easily define custom slice types.

extern crate proc_macro;

use proc_macro::TokenStream;

use crate::defs::Definitions;

pub(crate) mod attrs;
pub(crate) mod codegen;
pub(crate) mod defs;

#[proc_macro]
pub fn define_slice_types_pair(input: TokenStream) -> TokenStream {
    let file: syn::File = syn::parse(input).expect("Failed to parse input token stream");
    let defs = Definitions::from_file(file).expect("Failed to load definitions");
    let output = defs.generate();
    output.into()
}
