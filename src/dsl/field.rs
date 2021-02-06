use super::{
    analysis::{Analyzer, Normalizer},
    character_filters::CharacterFilterType,
    token_processing::TokenFilterType,
    tokenizers::TokenizerType,
};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Serialize)]
#[serde(into = "IndexMappingDTO")]
pub struct IndexMapping {
    fields: Vec<Field>,
}

#[derive(PartialEq, Eq, Clone, Serialize)]
pub struct Field {
    #[serde(skip)]
    name: String,
    #[serde(flatten)]
    field_type: FieldType,
    #[serde(serialize_with = "serialize_fields")]
    fields: Vec<Field>,
}

#[derive(PartialEq, Eq, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum FieldType {
    Binary,
    Boolean,
    #[serde(serialize_with = "serialize_keyword")]
    Keyword {
        normalizer: Normalizer,
    },
    Text {
        analyzer: Analyzer,
    },
    Date,
    Long,
    Integer,
    Short,
    Byte,
    Double,
    Float,
    HalfFloat,
    ScaledFloat,
    UnsignedLong,
}

fn serialize_keyword<S>(n: &Normalizer, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry("normalizer", &n.name)?;
    map.end()
}

fn serialize_text<S>(n: &Analyzer, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry("analyzer", &n.name)?;
    map.end()
}

fn serialize_fields<S>(fields: &[Field], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(fields.len()))?;
    for field in fields {
        map.serialize_entry(&field.name, field)?;
    }
    map.end()
}

trait Indexable {
    fn index_mapping() -> IndexMapping;
}

#[derive(Serialize)]
struct IndexMappingDTO {
    analysis: AnalysisDTO,
    mappings: MappingsDTO,
}

impl From<IndexMapping> for IndexMappingDTO {
    fn from(mapping: IndexMapping) -> Self {
        let mut analysis = AnalysisDTO {
            char_filter: HashMap::new(),
            tokenizer: HashMap::new(),
            filter: HashMap::new(),
            normalizer: HashMap::new(),
            analyzer: HashMap::new()
        };

        process_fields(&mapping.fields, &mut analysis);

        IndexMappingDTO {
            analysis: analysis,
            mappings: MappingsDTO {
                properties: mapping.fields,
            },
        }
    }
}

fn process_fields(fields: &[Field], analysis: &mut AnalysisDTO) {
    for field in fields {
        process_fields(&field.fields, analysis);
        match &field.field_type {
            FieldType::Keyword {normalizer: n} => {

            },
            FieldType::Text {analyzer: a} => {

            },
            _ => ()
        };
    }
}

#[derive(Serialize)]
struct AnalysisDTO {
    char_filter: HashMap<String, CharacterFilterType>,
    tokenizer: HashMap<String, TokenizerType>,
    filter: HashMap<String, TokenFilterType>,
    normalizer: HashMap<String, Normalizer>,
    analyzer: HashMap<String, Analyzer>,
}

#[derive(Serialize)]
struct MappingsDTO {
    #[serde(serialize_with = "serialize_fields")]
    properties: Vec<Field>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::{
        character_filters::{CharacterFilter, CharacterFilterType},
        tokenizers::{CharacterGroups, Tokenizer, TokenizerType},
    };
    use serde_json::{json, to_value};

    fn keyword() -> FieldType {
        FieldType::Keyword {
            normalizer: Normalizer {
                name: "my_normalizer".to_string(),
                character_filters: vec![CharacterFilter {
                    name: "my_char_filter".to_string(),
                    character_filter_type: CharacterFilterType::Mapping {
                        mappings: vec![("-".to_string(), "_".to_string())],
                    },
                }],
                tokenizer: Tokenizer {
                    name: "my_tokenizer".to_string(),
                    tokenizer_type: TokenizerType::CharacterGroup {
                        tokenize_on_chars: vec![CharacterGroups::Whitespace],
                    },
                },
            },
        }
    }

    #[test]
    fn keyword_serialization() {
        assert_eq!(
            to_value(keyword()).unwrap(),
            json!({
                "type": "keyword",
                "normalizer": "my_normalizer"
            })
        )
    }

    #[test]
    fn index_mapping() {
        let index = IndexMapping {
            fields: vec![
                Field {
                    name: "number".to_string(),
                    field_type: FieldType::Float,
                    fields: vec![Field {
                        name: "int".to_string(),
                        field_type: FieldType::Integer,
                        fields: vec![],
                    }],
                },
                Field {
                    name: "keyword".to_string(),
                    field_type: keyword(),
                    fields: vec![],
                },
            ],
        };
        let expected = json!({
            "analysis": {
                "char_filter": {
                    "my_char_filter": {
                        "type": "mapping",
                        "mappings": [
                            "- => _"
                        ]
                    }
                },
                "tokenizer": {
                    "my_tokenizer": {
                        "type": "char_group",
                        "tokenize_on_chars": [
                            "whitespace"
                        ]
                    }
                },
                "normalizer": {
                    "my_normalizer": {
                        "char_filter": [
                            "my_char_filter"
                        ],
                        "tokenizer": "my_tokenizer"
                    }
                }
            },
            "mappings": {
                "properties": {
                    "number": {
                        "type": "float",
                        "fields": {
                            "int": {
                                "type": "integer",
                                "fields": {}
                            }
                        }
                    },
                    "keyword": {
                        "type": "keyword",
                        "normalizer": "my_normalizer",
                        "fields": {}
                    }
                }
            }
        });
        assert_eq!(to_value(&index).unwrap(), expected)
    }
}
