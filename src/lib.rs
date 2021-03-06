use serde::{Deserialize, Serialize};

const DELIMITERS: &str = "(){}[].,:;";
const OPERATORS: &str = "+-*/=!<>";
const DOUBLE_CHAR_OPERATORS: [&str; 10] =
    ["==", "!=", "<=", ">=", "++", "--", "+=", "-=", "*=", "/="];
const LINE_TOKENS: [Type; 3] = [Type::Comment, Type::Name, Type::String];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    OpeningKw,
    ClosingKw,
    RegularKw,
    Number,
    Operator,
    String,
    Delimiter,
    Identifier,
    NewLine,
    Comment,
    Name,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub t: Type,
    pub c: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Keywords {
    opening: Vec<String>,
    closing: Vec<String>,
    regular: Vec<String>,
    comment: String,
    name: String,
    closing_prefix: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub indent: u32,
    pub tokens: Vec<Token>,
}

pub fn tokenize(input_str: &str, lang_config_str: &str, capitalize_keywords: bool) -> Vec<Token> {
    let keywords: Keywords = serde_json::from_str(lang_config_str).unwrap();

    let mut tokens: Vec<Token> = Vec::new();
    let mut token_type: Type = Type::None;
    let mut token_content: String = String::new();

    let mut last_char: char = ' ';

    for c in input_str.chars() {
        if c.is_whitespace() {
            if token_type == Type::String {
                token_content += &c.to_string();
            } else if LINE_TOKENS.contains(&token_type) {
                if c == '\n' {
                    tokens.push(Token {
                        t: token_type,
                        c: token_content.clone(),
                    });
                    tokens.push(Token {
                        t: Type::NewLine,
                        c: String::new(),
                    });

                    token_type = Type::None;
                    token_content.clear();
                } else {
                    token_content += &c.to_string();
                }
            } else {
                if !token_content.is_empty() {
                    if token_type == Type::Identifier {
                        let data = construct_identifier_token(
                            &token_content,
                            token_type,
                            &keywords,
                            capitalize_keywords,
                        );
                        token_type = data.0;
                        token_content = data.1;
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
            }
        } else if c == '"' {
            if !token_content.is_empty() {
                if last_char != '\\' {
                    if token_type == Type::String {
                        token_content = format!("\"{}\"", &token_content);
                    } else if token_type == Type::Identifier {
                        let data = construct_identifier_token(
                            &token_content,
                            token_type,
                            &keywords,
                            capitalize_keywords,
                        );
                        token_type = data.0;
                        token_content = data.1;
                    }

                    tokens.push(Token {
                        t: token_type,
                        c: token_content.clone(),
                    });
                    token_type = Type::None;
                    token_content.clear();
                } else {
                    token_content += &c.to_string();
                }
            } else if token_content.is_empty() {
                token_type = Type::String;
            }
        } else if c.is_alphabetic() || c == '_' {
            if token_content.is_empty() && !LINE_TOKENS.contains(&token_type) {
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
        } else if c.is_digit(10) {
            if token_content.is_empty() && !LINE_TOKENS.contains(&token_type) {
                token_type = Type::Number;
            }

            token_content += &c.to_string();
        } else if DELIMITERS.contains(c) {
            if token_type == Type::Identifier {
                let data = construct_identifier_token(
                    &token_content,
                    token_type,
                    &keywords,
                    capitalize_keywords,
                );
                token_type = data.0;
                token_content = data.1;
            }

            if LINE_TOKENS.contains(&token_type) {
                token_content += &c.to_string();
            } else {
                if !token_content.is_empty() {
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
            }
        } else if OPERATORS.contains(c) {
            if LINE_TOKENS.contains(&token_type) {
                token_content += &c.to_string();
            } else {
                if !token_content.is_empty() {
                    if token_type == Type::Identifier {
                        let data = construct_identifier_token(
                            &token_content,
                            token_type,
                            &keywords,
                            capitalize_keywords,
                        );
                        token_type = data.0;
                        token_content = data.1;
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
        } else if LINE_TOKENS.contains(&token_type) {
            token_content += &c.to_string();
        }

        last_char = c;
    }

    if !token_content.is_empty() {
        if token_type == Type::Identifier {
            let data = construct_identifier_token(
                &token_content,
                token_type,
                &keywords,
                capitalize_keywords,
            );
            token_type = data.0;
            token_content = data.1;
        }

        tokens.push(Token {
            t: token_type,
            c: token_content,
        });
    }

    combine_tokens(&mut tokens);
    set_closing_name_keyword(&mut tokens, &keywords);

    tokens
}

fn construct_identifier_token(
    token_content: &str,
    token_type: Type,
    keywords: &Keywords,
    capitalize_keywords: bool,
) -> (Type, String) {
    let mut content = token_content.to_string();
    let mut t_type = token_type;
    let lc = content.to_lowercase();

    let l = [
        (Type::OpeningKw, &keywords.opening),
        (Type::ClosingKw, &keywords.closing),
        (Type::RegularKw, &keywords.regular),
    ];

    for (t, k) in l.iter() {
        let index = k.iter().position(|r| r.to_lowercase() == lc);

        if index != None {
            if capitalize_keywords {
                content = k[index.unwrap()].clone();
            }

            return (*t, content);
        }
    }

    // So it's not a keyword...
    if lc == keywords.comment.to_lowercase() {
        if capitalize_keywords {
            content = keywords.comment.clone();
        }

        t_type = Type::Comment;
    } else if lc == keywords.name.to_lowercase() {
        if capitalize_keywords {
            content = keywords.name.clone();
        }

        t_type = Type::Name;
    }

    (t_type, content)
}

fn combine_tokens(tokens: &mut Vec<Token>) {
    if tokens.is_empty() {
        return;
    }

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
}

fn set_closing_name_keyword(tokens: &mut Vec<Token>, keywords: &Keywords) {
    let mut closing_name_kw = String::new();

    for i in 0..tokens.len() {
        if tokens[i].t == Type::Name {
            let name: String = tokens[i]
                .c
                .strip_prefix(&format!("{}:", &keywords.name))
                .unwrap()
                .split_whitespace()
                .collect();
            closing_name_kw = format!("{}{}", &keywords.closing_prefix, name);
        } else if tokens[i].t == Type::Identifier && tokens[i].c == closing_name_kw {
            tokens[i].t = Type::ClosingKw;
        }
    }
}

pub fn indent(tokens: &[Token]) -> Vec<Line> {
    let mut indent_level: u32 = 0;
    let mut increase_indent = false;
    let mut line_tokens: Vec<Token> = Vec::new();
    let mut indented_lines: Vec<Line> = Vec::new();

    for token in tokens.iter() {
        match token.t {
            Type::NewLine => {
                indented_lines.push(Line {
                    tokens: line_tokens.clone(),
                    indent: indent_level,
                });

                line_tokens.clear();

                if increase_indent {
                    indent_level += 1;
                    increase_indent = false;
                }
            }
            Type::OpeningKw => {
                increase_indent = true;
                line_tokens.push(token.clone());
            }
            Type::ClosingKw => {
                if indent_level > 0 {
                    indent_level -= 1;
                }

                line_tokens.push(token.clone());
                increase_indent = false;
            }
            _ => line_tokens.push(token.clone()),
        }
    }

    if !line_tokens.is_empty() {
        indented_lines.push(Line {
            tokens: line_tokens,
            indent: indent_level,
        });
    }

    indented_lines
}
