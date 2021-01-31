use serde::Deserialize;

const IDENTIFIER_CHARS: &str = "abcdefghijklmnñopqrstuvwxyzABCDEFGHIJKLMNÑOPQRSTUVWXYZ_";

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
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub t: Type,
    pub c: String,
}

#[derive(Deserialize, Debug)]
struct Keywords {
    opening: Vec<String>,
    closing: Vec<String>,
    regular: Vec<String>,
}

pub fn tokenize<'a> (input_str: &'a str, lang_config_str: &'static str) -> Vec<Token> {
    let _keywords: Keywords = serde_json::from_str(lang_config_str).unwrap();

    let mut tokens: Vec<Token> = Vec::new();
    let mut token_type: Type = Type::None;
    let mut token_content: String = String::new();

    for c in input_str.chars() {
        if IDENTIFIER_CHARS.contains(c) {
            if token_content.is_empty() {
                token_type = Type::Identifier;
            }

            token_content += &c.to_string();
        } else if c == ' ' {
            if !token_content.is_empty() {
                tokens.push(Token {t: token_type, c: token_content.clone()});
                token_type = Type::None;
                token_content.clear();
            }
        }
    }

    if !token_content.is_empty() {
        tokens.push(Token {t: token_type, c: token_content.clone()});
    }

    tokens
}
