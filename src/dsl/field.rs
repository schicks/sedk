use super::analysis::{Normalizer,Analyzer};

#[derive(PartialEq, Eq)]
struct IndexMapping {
    fields: Vec<Field>
}


#[derive(PartialEq, Eq)]
struct Field {
    name: String,
    field_type: FieldType,
    fields: Vec<Field>
}

#[derive(PartialEq, Eq)]
enum FieldType {
    Binary,
    Boolean,
    Keyword(Normalizer),
    Text(Analyzer),
    Date,
    Long,
    Integer,
    Short,
    Byte,
    Double,
    Float,
    HalfFloat,
    ScaledFloat,
    UnsignedLong
}

trait Indexable {
    fn index_mapping() -> IndexMapping;
}