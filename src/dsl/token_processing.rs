use serde::ser::{Serializer};
use serde::Serialize;

#[derive(PartialEq, Eq, Hash)]
pub struct TokenFilter {
    name: String,
    filter_type: TokenFilterType
}

#[derive(PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
enum TokenFilterType {
    Lowercase,
    Stemmer {language: StemmerLanguage},
    SynonymGraph {
        expand: bool,
        lenient: bool,
        synonyms: Vec<Synonym>
    },
    FlattenGraph,
    Reverse,
    Shingle {
        max_shingle_size: u8,
        min_shingle_size: u8,
        output_unigrams: bool,
        output_unigrams_if_no_shingles: bool,
        token_separator: String,
        filler_token: String,
    }
}

#[derive(PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
enum StemmerLanguage {
    Arabic,
    Armenian,
    Basque,
    Bengali,
    Brazilian,
    Bulgarian,
    Catalan,
    Czech,
    Danish,
    Dutch,
    DutchKp,
    English,
    LightEnglish,
    Lovins,
    MinimalEnglish,
    Porter2,
    PossessiveEnglish,
    Estonian,
    Finnish,
    LightFinnish,
    French,
    LightFrench,
    MinimalFrench,
    Galician,
    MinimalGalician,
    German,
    LightGerman,
    German2,
    MinimalGerman,
    Greek,
    Hindi,
    Hungarian,
    LightHungarian,
    Indonesian,
    Irish,
    Italian,
    LightItalian,
    Sorani,
    Latvian,
    Lithuanian,
    Norwegian,
    LightNorwegian,
    MinimalNorwegian,
    LightNynorsk,
    MinimalNynorsk,
    Portueguese,
    LightPortuguese,
    MinimalPortuguese,
    PortugueseRslp,
    Romanian,
    Russian,
    LightRussian,
    Spanish,
    LightSpanish,
    Swedish,
    LightSwedish,
    Turkish
}

#[derive(PartialEq, Eq, Hash)]
struct Synonym {
    from: Vec<String>,
    to: Vec<String>
}

impl Serialize for Synonym {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        format!("{} => {}", self.from.join(","), self.to.join(",")).serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};
    #[test]
    fn lowercase() {
        let tokenizer = TokenFilterType::Lowercase;
        let expected = json!({
            "type": "lowercase"
        });
        assert_eq!(
            to_value(&tokenizer).unwrap(),
            expected
        )
    }

    #[test]
    fn stemmer() {
        let tokenizer = TokenFilterType::Stemmer {language: StemmerLanguage::Porter2};
        let expected = json!({
            "type": "stemmer",
            "language": "porter2"
        });
        assert_eq!(
            to_value(&tokenizer).unwrap(),
            expected
        )
    }

    #[test]
    fn synonym_graph() {
        let tokenizer = TokenFilterType::SynonymGraph {
            expand: false,
            lenient: true,
            synonyms: vec![
                Synonym {
                    from: vec!["short", "small", "little"].iter().map(|s| s.to_string()).collect(),
                    to: vec!["small"].iter().map(|s| s.to_string()).collect()
                }
            ]
        };
        let expected = json!({
            "type": "synonym_graph",
            "expand": false,
            "lenient": true,
            "synonyms": [
                "short,small,little => small"
            ]
        });
        assert_eq!(
            to_value(&tokenizer).unwrap(),
            expected
        )
    }

    #[test]
    fn shingle() {
        let tokenizer = TokenFilterType::Shingle {
            filler_token: "-".to_string(),
            max_shingle_size: 2,
            min_shingle_size: 1,
            output_unigrams: false,
            output_unigrams_if_no_shingles: true,
            token_separator: "_".to_string()
        };
        let expected = json!({
            "type": "shingle",
            "filler_token": "-",
            "max_shingle_size": 2,
            "min_shingle_size": 1,
            "output_unigrams": false,
            "output_unigrams_if_no_shingles": true,
            "token_separator": "_"
        });
        assert_eq!(
            to_value(&tokenizer).unwrap(),
            expected
        )
    }
}