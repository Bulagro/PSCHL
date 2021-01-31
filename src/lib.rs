use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub enum Type {
    OKeyword,
    CKeyword,
    RKeyword,
    Number,
    Operator,
    String,
    Delimiter,
    Identifier,
    NewLine,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub t: Type,
    pub c: String,
}

impl Token {
    pub fn new(t: Type, c: String) -> Self {
        Token {t, c}
    }
}

#[derive(Deserialize, Debug)]
struct Keywords {
    opening: Vec<String>,
    closing: Vec<String>,
    regular: Vec<String>,
}

pub fn tokenize<'a> (input_str: &'a str, lang_config_str: &'static str) -> Vec<Token> {
    if input_str == "" {}

    let keywords: Keywords = serde_json::from_str(lang_config_str).unwrap();
    let tokens: Vec<Token> = Vec::new();



    tokens
}
