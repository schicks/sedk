use serde::ser::{Serializer};
use serde::Serialize;

#[derive(PartialEq, Eq, Hash)]
pub struct Tokenizer {
    pub name: String,
    pub tokenizer_type: TokenizerType,
}

#[derive(PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "type")]
pub enum TokenizerType {
    #[serde(rename = "char_group")]
    CharacterGroup {
        tokenize_on_chars: Vec<CharacterGroups>
    },
}

#[derive(PartialEq, Eq, Hash)]
pub enum CharacterGroups {
    Whitespace,
    Letter,
    Digit,
    Punctuation,
    Symbol,
    Arbitrary(char),
}

impl Serialize for CharacterGroups {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        match self {
            CharacterGroups::Whitespace => "whitespace".to_string(),
            CharacterGroups::Letter => "letter".to_string(),
            CharacterGroups::Digit => "digit".to_string(),
            CharacterGroups::Punctuation => "punctuation".to_string(),
            CharacterGroups::Symbol => "symbol".to_string(),
            CharacterGroups::Arbitrary(c) => c.to_string(),
        }.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};
    #[test]
    fn char_group() {
        let tokenizer = TokenizerType::CharacterGroup {
            tokenize_on_chars: vec![
                CharacterGroups::Whitespace,
                CharacterGroups::Punctuation,
                CharacterGroups::Arbitrary('-'),
                CharacterGroups::Arbitrary('\n')
            ]
        };
        let expected = json!({
            "type": "char_group",
            "tokenize_on_chars": [
                "whitespace",
                "punctuation",
                "-",
                "\n"
            ]
        });
        assert_eq!(
            to_value(&tokenizer).unwrap(),
            expected
        )
    }
}