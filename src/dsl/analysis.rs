use super::token_processing::TokenFilter;
use super::character_filters::CharacterFilter;
use super::tokenizers::Tokenizer;

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