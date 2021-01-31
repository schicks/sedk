#[derive(PartialEq, Eq, Hash)]
pub struct TokenFilter {
    name: String,
    filter_type: TokenFilterType
}

#[derive(PartialEq, Eq, Hash)]
enum TokenFilterType {
    Lowercase,
    Stemmer(StemmerLanguage),
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

#[derive(PartialEq, Eq, Hash)]
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
    DutchKP,
    English,
    EnglishLight,
    EnglishLovins,
    EnglishMinimal,
    EnglishPorter2,
    EnglishPossesive,
    Estonian,
    Finnish,
    FinnishLight,
    French,
    FrenchLight,
    FrenchMinimal,
    Galician,
    GalicianMinimal,
    German,
    GermanLight,
    German2,
    GermanMinimal,
    Greek,
    Hindi,
    Hungarian,
    HungarianLight,
    Indonesian,
    Irish,
    Italian,
    ItalianLight,
    Sorani,
    Latvian,
    Lithuanian,
    Norwegian,
    NorwegianLight,
    NorwegianMinimal,
    NynorskLight,
    NynorskMinimal,
    Portueguese,
    PortugueseLight,
    PortugueseMinimal,
    PortugueseRSLP,
    Romanian,
    Russian,
    RussianLight,
    Spanish,
    SpanishLight,
    Swedish,
    SwedishLight,
    Turkish
}

#[derive(PartialEq, Eq, Hash)]
struct Synonym {
    from: Vec<String>,
    to: Vec<String>
}