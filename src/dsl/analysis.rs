use super::token_processing::TokenFilter;
use super::character_filters::CharacterFilter;
use super::tokenizers::Tokenizer;
use serde::Serialize;
use serde::ser::Serializer;
use serde_json::json;

#[derive(PartialEq, Eq, Hash)]
pub struct Analyzer {
    name: String,
    character_filters: Vec<CharacterFilter>,
    tokenizer: Tokenizer,
    token_filters: Vec<TokenFilter>
}

impl Serialize for Analyzer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        json!({
            "char_filter": self.character_filters.iter().map(|cf| cf.name.clone()).collect::<Vec<String>>(),
            "tokenizer": self.tokenizer.name,
            "filter": self.token_filters.iter().map(|tf| tf.name.clone()).collect::<Vec<String>>()
        }).serialize(serializer)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Normalizer {
    name: String,
    character_filters: Vec<CharacterFilter>,
    tokenizer: Tokenizer
}

impl Serialize for Normalizer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        json!({
            "char_filter": self.character_filters.iter().map(|cf| cf.name.clone()).collect::<Vec<String>>(),
            "tokenizer": self.tokenizer.name
        }).serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::character_filters::CharacterFilterType;
    use crate::dsl::tokenizers::{TokenizerType,CharacterGroups};
    use serde_json::to_value;

    #[test]
    fn analyzer() {
        let analyzer = Analyzer {
            name: "my_analyzer".to_string(),
            character_filters: vec![
                CharacterFilter {
                    name: "my_char_filter".to_string(), 
                    character_filter_type: CharacterFilterType::Mapping {
                        mappings: vec![]
                    }
                }
            ],
            tokenizer: Tokenizer {
                name: "my_tokenizer".to_string(),
                tokenizer_type: TokenizerType::CharacterGroup {
                    tokenize_on_chars: vec![CharacterGroups::Whitespace]
                }
            },
            token_filters: vec![]
        };
        let expected = json!({
            "char_filter": vec!["my_char_filter"],
            "tokenizer": "my_tokenizer",
            "filter": []
        });
        assert_eq!(to_value(&analyzer).unwrap(), expected)
    }

    #[test]
    fn normalizer() {
        let normalizer = Normalizer {
            name: "my_normalizer".to_string(),
            character_filters: vec![
                CharacterFilter {
                    name: "my_char_filter".to_string(), 
                    character_filter_type: CharacterFilterType::Mapping {
                        mappings: vec![]
                    }
                }
            ],
            tokenizer: Tokenizer {
                name: "my_tokenizer".to_string(),
                tokenizer_type: TokenizerType::CharacterGroup {
                    tokenize_on_chars: vec![CharacterGroups::Whitespace]
                }
            }
        };
        let expected = json!({
            "char_filter": vec!["my_char_filter"],
            "tokenizer": "my_tokenizer"
        });
        assert_eq!(to_value(&normalizer).unwrap(), expected)
    }
}