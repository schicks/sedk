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
    pub fields: Vec<Field>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub struct Field {
    #[serde(skip)]
    pub name: String,
    #[serde(flatten)]
    pub field_type: FieldType,
    #[serde(serialize_with = "serialize_fields")]
    pub fields: Vec<Field>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FieldType {
    Binary,
    Boolean,
    #[serde(serialize_with = "serialize_keyword")]
    Keyword {
        normalizer: Normalizer,
    },
    #[serde(serialize_with = "serialize_text")]
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

pub trait Indexable {
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
            analyzer: HashMap::new(),
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
            FieldType::Keyword { normalizer: n } => {
                analysis.normalizer.insert(n.name.clone(), n.clone());
                analysis
                    .tokenizer
                    .insert(n.tokenizer.name.clone(), n.tokenizer.tokenizer_type.clone());
                for cf in &n.character_filters {
                    analysis
                        .char_filter
                        .insert(cf.name.clone(), cf.character_filter_type.clone());
                }
            }
            FieldType::Text { analyzer: a } => {
                analysis.analyzer.insert(a.name.clone(), a.clone());
                analysis
                    .tokenizer
                    .insert(a.tokenizer.name.clone(), a.tokenizer.tokenizer_type.clone());
                for cf in &a.character_filters {
                    analysis
                        .char_filter
                        .insert(cf.name.clone(), cf.character_filter_type.clone());
                }
                for tf in &a.token_filters {
                    analysis
                        .filter
                        .insert(tf.name.clone(), tf.filter_type.clone());
                }
            }
            _ => (),
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
    use pretty_assertions::assert_eq;
    use serde_json::{json, to_value};

    fn normalizer() -> Normalizer {
        Normalizer {
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
        }
    }

    fn analyzer() -> Analyzer {
        Analyzer::from_normalizer(&normalizer(), "my_analyzer".to_string(), vec![])
    }

    #[test]
    fn keyword_serialization() {
        assert_eq!(
            to_value(FieldType::Keyword {
                normalizer: normalizer()
            })
            .unwrap(),
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
                    field_type: FieldType::Keyword {
                        normalizer: normalizer(),
                    },
                    fields: vec![Field {
                        name: "text".to_string(),
                        field_type: FieldType::Text {
                            analyzer: analyzer(),
                        },
                        fields: vec![],
                    }],
                },
            ],
        };
        let expected = json!({
            "analysis": {
                "analyzer": {
                    "my_analyzer": {
                        "char_filter": [
                            "my_char_filter"
                        ],
                        "tokenizer": "my_tokenizer",
                        "filter": []
                    }
                },
                "filter": {},
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
                        "fields": {
                            "text": {
                                "type": "text",
                                "analyzer": "my_analyzer",
                                "fields": {}
                            }
                        }
                    }
                }
            }
        });
        assert_eq!(to_value(&index).unwrap(), expected)
    }
}
