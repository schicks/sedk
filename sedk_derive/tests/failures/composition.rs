use sedk::{IntoFields, Field};
use sedk_derive::IntoFields;

pub struct Child {
    integer: i32,
    string: String
}

#[derive(IntoFields)]
pub struct Parent {
    child: Child
}

fn main() {
}