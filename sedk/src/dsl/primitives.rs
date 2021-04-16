use super::{
    analysis::{Analyzer, Normalizer},
    field::{Field, FieldType},
};

pub trait IntoFields {
    fn into_fields(name: Option<&str>) -> Vec<Field>;
}

impl IntoFields for String {
    fn into_fields(name: Option<&str>) -> Vec<Field> {
        match name {
            None => Vec::new(), // is this too quiet?
            Some(s) => vec![Field {
                name: s.to_owned(),
                fields: vec![Field {
                    name: s.to_owned() + ".keyword",
                    fields: Vec::new(),
                    field_type: FieldType::Keyword {
                        normalizer: Normalizer::default(),
                    },
                }],
                field_type: FieldType::Text {
                    analyzer: Analyzer::default(),
                },
            }],
        }
    }
}

// most primitives have nearly identical implementations of into_fields
macro_rules! from_type {
    ($t:expr) => {
        fn into_fields(name: Option<&str>) -> Vec<Field> {
            match name {
                None => Vec::new(),
                Some(s) => vec![Field {
                    name: s.to_owned(),
                    fields: Vec::new(),
                    field_type: $t,
                }],
            }
        }
    };
}
impl IntoFields for i64 {
    from_type!(FieldType::Long);
}
impl IntoFields for i32 {
    from_type!(FieldType::Integer);
}
impl IntoFields for i16 {
    from_type!(FieldType::Short);
}
impl IntoFields for u64 {
    from_type!(FieldType::UnsignedLong);
}
impl IntoFields for f64 {
    from_type!(FieldType::Double);
}
impl IntoFields for f32 {
    from_type!(FieldType::Float);
}
impl IntoFields for bool {
    from_type!(FieldType::Boolean);
}
