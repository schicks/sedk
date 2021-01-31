use super::token_processing::TokenFilter;
use super::text_processing::{CharacterFilter, Tokenizer};

#[derive(PartialEq, Eq, Hash)]
pub struct Analyzer {
    name: String,
    character_filters: Vec<CharacterFilter>,
    tokenizer: Tokenizer,
    token_filters: Vec<TokenFilter>
}

#[derive(PartialEq, Eq, Hash)]
pub struct Normalizer {
    name: String,
    character_filters: Vec<CharacterFilter>,
    tokenizer: Tokenizer
}