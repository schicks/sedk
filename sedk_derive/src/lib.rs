/// # Examples
/// ```rust
/// use sedk::{IntoFields, Indexable, IndexMapping, Field, FieldType, Analyzer, Normalizer};
/// use sedk_derive::{IntoFields, Indexable};

/// #[derive(IntoFields)]
/// pub struct Child {
///     integer: i32,
///     string: String
/// }
///
/// #[derive(IntoFields, Indexable)]
/// pub struct Parent {
///     nested: Child
/// }

/// fn main() {
///     let fields = Child::into_fields(None);
///     assert_eq!(
///         fields,
///         vec![
///             Field {
///                 name: "integer".to_owned(), 
///                 field_type: FieldType::Integer, 
///                 fields: Vec::new()
///             },
///             Field {
///                 name: "string".to_owned(),
///                 field_type: FieldType::Text {analyzer: Analyzer::default()},
///                 fields: vec![
///                     Field {
///                         name: "string.keyword".to_owned(),
///                         field_type: FieldType::Keyword {normalizer: Normalizer::default()},
///                         fields: Vec::new()
///                     }
///                 ]
///             }
///         ]
///     );
///     let parent_mapping = Parent::index_mapping();
///     assert_eq!(
///         parent_mapping.fields,
///         vec![
///             Field {
///                 name: "nested.integer".to_owned(), 
///                 field_type: FieldType::Integer, 
///                 fields: Vec::new()
///             },
///             Field {
///                 name: "nested.string".to_owned(),
///                 field_type: FieldType::Text {analyzer: Analyzer::default()},
///                 fields: vec![
///                     Field {
///                         name: "nested.string.keyword".to_owned(),
///                         field_type: FieldType::Keyword {normalizer: Normalizer::default()},
///                         fields: Vec::new()
///                     }
///                 ]
///             }
///         ]
///     );
/// }
/// ```
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