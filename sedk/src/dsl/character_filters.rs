use serde::ser::Serializer;
use serde::Serialize;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CharacterFilter {
    pub name: String,
    pub character_filter_type: CharacterFilterType,
}

#[derive(PartialEq, Eq, Hash, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CharacterFilterType {
    HtmlStrip {
        escaped_tags: Vec<String>,
    },
    Mapping {
        #[serde(serialize_with = "serialize_mappings")]
        mappings: Vec<(String, String)>,
    },
    PatternReplace {
        pattern: String,
        replacement: String,
        #[serde(serialize_with = "serialize_flags")]
        flags: Vec<RegexFlag>,
    },
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum RegexFlag {
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

fn serialize_flags<S>(flags: &Vec<RegexFlag>, serializer: S) -> Result<S::Ok, S::Error>
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

fn serialize_mappings<S>(mappings: &Vec<(String, String)>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(
        mappings
            .iter()
            .map(|(from, to)| format!("{} => {}", from, to)),
    )
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
        assert_eq!(to_value(&char_filter).unwrap(), expected)
    }

    #[test]
    fn html_strip() {
        let char_filter = CharacterFilterType::HtmlStrip {
            escaped_tags: vec!["b".to_owned()],
        };
        let expected = json!({
            "type": "html_strip",
            "escaped_tags": ["b"]
        });
        assert_eq!(to_value(&char_filter).unwrap(), expected)
    }

    #[test]
    fn mapping() {
        let char_filter = CharacterFilterType::Mapping {
            mappings: vec![
                ("a".to_owned(), "b".to_owned()),
                ("c".to_owned(), "d".to_owned()),
            ],
        };
        let expected = json!({
            "type": "mapping",
            "mappings": [
                "a => b",
                "c => d"
            ]
        });

        assert_eq!(to_value(&char_filter).unwrap(), expected)
    }
}
