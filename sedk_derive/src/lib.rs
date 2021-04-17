mod indexable;
mod into_fields;
use indexable::impl_indexable;
use into_fields::impl_into_fields;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Indexable)]
pub fn indexable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_indexable(input).into()
}

#[proc_macro_derive(IntoFields)]
pub fn into_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_into_fields(input).into()
}