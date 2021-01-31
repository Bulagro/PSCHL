use serde::Deserialize;

const IDENTIFIER_CHARS: &str = "abcdefghijklmnñopqrstuvwxyzABCDEFGHIJKLMNÑOPQRSTUVWXYZ_";
const DIGITS: &str = "0123456789";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    OKeyword, // Opening
    CKeyword, // Closing
    RKeyword, // Regular
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

fn get_keyword_type_if_applicable<'a>(token_content: &'a str, keywords: &'a Keywords) -> Type {
    let token_content = token_content.to_string();
    let l = [
        (Type::OKeyword, &keywords.opening),
        (Type::CKeyword, &keywords.closing),
        (Type::RKeyword, &keywords.regular),
    ];

    for (t, k) in l.iter() {
        if k.contains(&token_content) {
            return *t;
        }
    }

    Type::Identifier
}

pub fn tokenize<'a>(input_str: &'a str, lang_config_str: &'static str) -> Vec<Token> {
    let keywords: Keywords = serde_json::from_str(lang_config_str).unwrap();

    let mut tokens: Vec<Token> = Vec::new();
    let mut token_type: Type = Type::None;
    let mut token_content: String = String::new();

    for c in input_str.chars() {
        if c == ' ' || c == '\n' {
            if !token_content.is_empty() {
                if token_type == Type::Identifier {
                    token_type = get_keyword_type_if_applicable(&token_content, &keywords);
                }

                tokens.push(Token {
                    t: token_type,
                    c: token_content.clone(),
                });
                token_type = Type::None;
                token_content.clear();
            }

            if c == '\n' {
                tokens.push(Token {
                    t: Type::NewLine,
                    c: String::new(),
                });
            }
        } else if IDENTIFIER_CHARS.contains(c) {
            if token_content.is_empty() {
                token_type = Type::Identifier;
            } else if token_type == Type::Number {
                tokens.push(Token {
                    t: token_type,
                    c: token_content.clone(),
                });
                token_type = Type::Identifier;
                token_content.clear();
            }

            token_content += &c.to_string();
        } else if DIGITS.contains(c) {
            if token_content.is_empty() {
                token_type = Type::Number;
            }

            token_content += &c.to_string();
        }
    }

    if !token_content.is_empty() {
        if token_type == Type::Identifier {
            token_type = get_keyword_type_if_applicable(&token_content, &keywords);
        }

        tokens.push(Token {
            t: token_type,
            c: token_content.clone(),
        });
    }

    tokens
}
