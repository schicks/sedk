use sedk::{IntoFields, Indexable, IndexMapping, Field, FieldType, Analyzer, Normalizer};
use sedk_derive::{IntoFields, Indexable};

#[derive(IntoFields, Indexable)]
pub struct Object {
    integer: i32,
    string: String
}

fn main() {
    let fields = Object::into_fields(None);
    let mapping = Object::index_mapping();
    assert_eq!(
        fields,
        vec![
            Field {
                name: "integer".to_owned(), 
                field_type: FieldType::Integer, 
                fields: Vec::new()
            },
            Field {
                name: "string".to_owned(),
                field_type: FieldType::Text {analyzer: Analyzer::default()},
                fields: vec![
                    Field {
                        name: "string.keyword".to_owned(),
                        field_type: FieldType::Keyword {normalizer: Normalizer::default()},
                        fields: Vec::new()
                    }
                ]
            }
        ]
    );

    assert_eq!(fields, mapping.fields);
}