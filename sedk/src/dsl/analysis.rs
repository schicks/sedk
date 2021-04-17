use super::character_filters::CharacterFilter;
use super::token_processing::TokenFilter;
use super::tokenizers::Tokenizer;
use serde::ser::Serializer;
use serde::Serialize;
use serde_json::json;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Analyzer {
    pub name: String,
    pub character_filters: Vec<CharacterFilter>,
    pub tokenizer: Tokenizer,
    pub token_filters: Vec<TokenFilter>,
}

impl Analyzer {
    pub fn from_normalizer(
        n: &Normalizer,
        name: String,
        token_filters: Vec<TokenFilter>,
    ) -> Analyzer {
        Analyzer {
            name: name,
            token_filters: token_filters,
            tokenizer: n.tokenizer(),
            character_filters: n.character_filters(),
        }
    }
}

impl Serialize for Analyzer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        json!({
            "char_filter": self.character_filters.iter().map(|cf| cf.name.clone()).collect::<Vec<String>>(),
            "tokenizer": self.tokenizer.name,
            "filter": self.token_filters.iter().map(|tf| tf.name.clone()).collect::<Vec<String>>()
        }).serialize(serializer)
    }
}

const DEFAULT_ANALYZER_NAME: &'static str = "sedk_default_normalizer";
impl Default for Analyzer {
    fn default() -> Self {
        Analyzer {
            name: DEFAULT_ANALYZER_NAME.to_owned(),
            character_filters: Vec::new(),
            token_filters: Vec::new(),
            tokenizer: Tokenizer::default(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Normalizer {
    pub name: String,
    pub character_filters: Vec<CharacterFilter>,
    pub tokenizer: Tokenizer,
}

impl Normalizer {
    fn tokenizer(&self) -> Tokenizer {
        self.tokenizer.clone()
    }

    fn character_filters(&self) -> Vec<CharacterFilter> {
        self.character_filters.clone()
    }
}

impl Serialize for Normalizer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        json!({
            "char_filter": self.character_filters.iter().map(|cf| cf.name.clone()).collect::<Vec<String>>(),
            "tokenizer": self.tokenizer.name
        }).serialize(serializer)
    }
}

const DEFAULT_NORMALIZER_NAME: &'static str = "sedk_default_normalizer";
impl Default for Normalizer {
    fn default() -> Self {
        Normalizer {
            name: DEFAULT_NORMALIZER_NAME.to_owned(),
            character_filters: Vec::new(),
            tokenizer: Tokenizer::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::character_filters::CharacterFilterType;
    use crate::dsl::tokenizers::{CharacterGroups, TokenizerType};
    use serde_json::to_value;

    #[test]
    fn analyzer() {
        let analyzer = Analyzer {
            name: "my_analyzer".to_string(),
            character_filters: vec![CharacterFilter {
                name: "my_char_filter".to_string(),
                character_filter_type: CharacterFilterType::Mapping { mappings: vec![] },
            }],
            tokenizer: Tokenizer {
                name: "my_tokenizer".to_string(),
                tokenizer_type: TokenizerType::CharacterGroup {
                    tokenize_on_chars: vec![CharacterGroups::Whitespace],
                },
            },
            token_filters: vec![],
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
            character_filters: vec![CharacterFilter {
                name: "my_char_filter".to_string(),
                character_filter_type: CharacterFilterType::Mapping { mappings: vec![] },
            }],
            tokenizer: Tokenizer {
                name: "my_tokenizer".to_string(),
                tokenizer_type: TokenizerType::CharacterGroup {
                    tokenize_on_chars: vec![CharacterGroups::Whitespace],
                },
            },
        };
        let expected = json!({
            "char_filter": vec!["my_char_filter"],
            "tokenizer": "my_tokenizer"
        });
        assert_eq!(to_value(&normalizer).unwrap(), expected)
    }
}
