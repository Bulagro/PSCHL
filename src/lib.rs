use serde::{Deserialize, Serialize};

const DELIMITERS: &str = "(){}[].,:;";
const OPERATORS: &str = "+-*/=!<>";
const DOUBLE_CHAR_OPERATORS: [&str; 10] =
	["==", "!=", "<=", ">=", "++", "--", "+=", "-=", "*=", "/="];
const LINE_TOKENS: [Type; 5] = [
	Type::Comment,
	Type::Name,
	Type::Input,
	Type::Output,
	Type::String,
];

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
	Comment,
	Name,
	Input,
	Output,
	None,
}

#[derive(Debug, PartialEq)]
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
	input: String,
	output: String,
	closing_prefix: String,
}

pub fn tokenize(input_str: &str, lang_config_str: &str) -> Vec<Token> {
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
			}
		} else if c == '"' {
			if !token_content.is_empty() {
				if last_char != '\\' {
					if token_type == Type::String {
						token_content = format!("\"{}\"", &token_content);
					} else if token_type == Type::Identifier {
						token_type = get_keyword_type_if_applicable(&token_content, &keywords);
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
		} else if c.is_alphabetic() {
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
				token_type = get_keyword_type_if_applicable(&token_content, &keywords);
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
		} else if LINE_TOKENS.contains(&token_type) {
			token_content += &c.to_string();
		}

		last_char = c;
	}

	if !token_content.is_empty() {
		if token_type == Type::Identifier {
			token_type = get_keyword_type_if_applicable(&token_content, &keywords);
		}

		tokens.push(Token {
			t: token_type,
			c: token_content,
		});
	}

	combine_tokens(tokens)
}

fn get_keyword_type_if_applicable(token_content: &str, keywords: &Keywords) -> Type {
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

	if token_content == keywords.comment {
		Type::Comment
	} else if token_content == keywords.name {
		Type::Name
	} else if token_content == keywords.input {
		Type::Input
	} else if token_content == keywords.output {
		Type::Output
	} else {
		Type::Identifier
	}
}

fn combine_tokens(tokens: Vec<Token>) -> Vec<Token> {
	if tokens.is_empty() {
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

pub fn get_updated_json_with_name(tokens: &[Token], lang_config_str: &str) -> String {
	let mut keywords: Keywords = serde_json::from_str(lang_config_str).unwrap();

	if tokens.is_empty() {
		return serde_json::to_string(&keywords).unwrap();
	}

	let name_index = tokens.iter().take_while(|token| token.t != Type::Name).count();

	let name: String = tokens[name_index]
		.c
		.strip_prefix(&format!("{}:", &keywords.name))
		.unwrap()
		.split_whitespace()
		.collect();

	if !name.is_empty() {
		keywords
			.closing
			.push(format!("{}{}", keywords.closing_prefix, name));
	}

	serde_json::to_string(&keywords).unwrap()
}
