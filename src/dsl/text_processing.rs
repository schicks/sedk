use serde::ser::Serializer;
use serde::Serialize;
use serde_json::to_string;

#[derive(PartialEq, Eq, Hash, Serialize)]
pub struct CharacterFilter {
    name: String,
    character_filter_type: CharacterFilterType,
}

#[derive(PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum CharacterFilterType {
    HTMLStrip(Vec<String>),
    Mapping(Vec<(String, String)>),
    PatternReplace {
        pattern: String,
        replacement: String,
        #[serde(serialize_with = "serializeFlags")]
        flags: Vec<RegexFlag>,
    },
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum RegexFlag {
    // https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html#field.summary
    CannonEq,
    CaseInsensitive,
    Comments,
    Dotall,
    Literal,
    Multiline,
    UnicodeCase,
    UnicodeCharacterClass,
    UnixLines,
}

impl From<&RegexFlag> for String {
    fn from(flag: &RegexFlag) -> String {
        match flag {
            RegexFlag::CannonEq => "CANNON_EQ",
            RegexFlag::CaseInsensitive => "CASE_INSENSITIVE",
            RegexFlag::Comments => "COMMENTS",
            RegexFlag::Dotall => "DOTALL",
            RegexFlag::Literal => "LITERAL",
            RegexFlag::Multiline => "MULTILINE",
            RegexFlag::UnicodeCase => "UNICODE_CASE",
            RegexFlag::UnicodeCharacterClass => "UNICODE_CHARACTER_CLASS",
            RegexFlag::UnixLines => "UNIX_LINES",
        }
        .to_string()
    }
}

fn serializeFlags<S>(flags: &Vec<RegexFlag>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = flags
        .iter()
        .map(|f| String::from(f))
        .collect::<Vec<String>>()
        .join("|");
    serializer.serialize_str(&s)
}

#[derive(PartialEq, Eq, Hash)]
pub struct Tokenizer {
    name: String,
    tokenizer_type: TokenizerType,
}

#[derive(PartialEq, Eq, Hash)]
enum TokenizerType {
    CharacterGroup(Vec<CharacterGroups>),
}

#[derive(PartialEq, Eq, Hash)]
enum CharacterGroups {
    Whitespace,
    Letter,
    Digit,
    Punctuation,
    Symbol,
    Arbitrary(char),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};
    #[test]
    fn pattern_replace() {
        let char_filter = CharacterFilterType::PatternReplace {
            flags: vec![RegexFlag::UnixLines, RegexFlag::UnicodeCharacterClass],
            pattern: "*".to_string(),
            replacement: "$1".to_string(),
        };
        let expected = json!({
            "type": "pattern_replace",
            "flags": "UNIX_LINES|UNICODE_CHARACTER_CLASS",
            "pattern": "*",
            "replacement": "$1"
        });
        assert_eq!(
            to_value(&char_filter).unwrap(),
            expected
        )
    }
}
