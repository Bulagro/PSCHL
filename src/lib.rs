use serde::Deserialize;

const IDENTIFIER_CHARS: &str = "abcdefghijklmnñopqrstuvwxyzABCDEFGHIJKLMNÑOPQRSTUVWXYZ_";
const DIGITS: &str = "0123456789";
const DELIMITERS: &str = "(){}[].,:;";
const OPERATORS: &str = "+-*/=!<>";
const DOUBLE_CHAR_OPERATORS: [&str; 10] =
    ["==", "!=", "<=", ">=", "++", "--", "+=", "-=", "*=", "/="];

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
        } else if DELIMITERS.contains(c) {
            if !token_content.is_empty() {
                if token_type == Type::Identifier {
                    token_type = get_keyword_type_if_applicable(&token_content, &keywords);
                }

                tokens.push(Token {
                    t: token_type,
                    c: token_content.clone(),
                });
            }

            tokens.push(Token {
                t: Type::Delimiter,
                c: String::from(c),
            });

            token_type = Type::None;
            token_content.clear();
        } else if OPERATORS.contains(c) {
            if !token_content.is_empty() {
                if token_type == Type::Identifier {
                    token_type = get_keyword_type_if_applicable(&token_content, &keywords);
                }

                tokens.push(Token {
                    t: token_type,
                    c: token_content.clone(),
                });
            }

            tokens.push(Token {
                t: Type::Operator,
                c: String::from(c),
            });

            token_type = Type::None;
            token_content.clear();
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

    combine_tokens(tokens)
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

fn combine_tokens(tokens: Vec<Token>) -> Vec<Token> {
    if tokens.len() == 0 {
        return tokens;
    }

    let mut tokens = tokens;
    let mut done = false;

    while !done {
        done = true;
        for i in 0..tokens.len() - 1 {
            let is_negative = &tokens[i].c == "-" && tokens[i + 1].t == Type::Number;

            let op: &str = &format!("{}{}", &tokens[i].c, &tokens[i + 1].c);
            let is_double_operator = OPERATORS.contains(&tokens[i].c)
                && OPERATORS.contains(&tokens[i + 1].c)
                && DOUBLE_CHAR_OPERATORS.contains(&op);

            let is_float = i < tokens.len() - 2
                && tokens[i].t == Type::Number
                && &tokens[i + 1].c == "."
                && tokens[i + 2].t == Type::Number;

            if is_negative {
                if (i > 0 && tokens[i - 1].t != Type::Number) || (i == 0) {
                    tokens[i + 1].c = format!("-{}", &tokens[i + 1].c);
                    tokens.remove(i);
                    done = false;
                    break;
                }
            } else if is_float {
                tokens[i].c = format!("{}.{}", &tokens[i].c, &tokens[i + 2].c);
                tokens.remove(i + 1);
                tokens.remove(i + 1);
                done = false;
                break;
            } else if is_double_operator {
                tokens[i].c = format!("{}{}", &tokens[i].c, &tokens[i + 1].c);
                tokens.remove(i + 1);
                done = false;
                break;
            }
        }
    }

    tokens
}
