use sedk::{IntoFields, Field, FieldType, Analyzer, Normalizer};
use sedk_derive::IntoFields;

#[derive(IntoFields)]
pub struct Child {
    integer: i32,
    string: String
}

#[derive(IntoFields)]
pub struct Parent {
    child: Child
}

fn main() {
    assert_eq!(
        Parent::into_fields(None),
        vec![
            Field {
                name: "child.integer".to_owned(), 
                field_type: FieldType::Integer, 
                fields: Vec::new()
            },
            Field {
                name: "child.string".to_owned(),
                field_type: FieldType::Text {analyzer: Analyzer::default()},
                fields: vec![
                    Field {
                        name: "child.string.keyword".to_owned(),
                        field_type: FieldType::Keyword {normalizer: Normalizer::default()},
                        fields: Vec::new()
                    }
                ]
            }
        ]
    )
}