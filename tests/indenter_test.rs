use pschl::*;

fn get_es_keywords() -> &'static str {
	r#"{
        "opening" : [
            "si",
            "mientras",
            "para"
        ],
        "closing" : [
            "finsi",
            "finmientras",
            "finpara"
        ],
        "regular" : [
            "entonces",
            "hasta"
        ],
        "comment" : "coment",
        "name" : "nombre",
        "input" : "entrada",
        "output" : "salida",
        "closing_prefix" : "fin"
    }"#
}

#[test]
fn test_empty_input() {
	let tokens: Vec<Token> = Vec::new();

	let expected: Vec<Line> = Vec::new();
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_new_line_adds_empty_line() {
	let tokens = tokenize("\n", get_es_keywords());

	let expected = vec![Line {
		indent: 0,
		tokens: Vec::new(),
	}];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_single_line() {
	let tokens = tokenize("single line 123 -", get_es_keywords());

	let expected = vec![Line {
		indent: 0,
		tokens: vec![
			Token {
				t: Type::Identifier,
				c: String::from("single"),
			},
			Token {
				t: Type::Identifier,
				c: String::from("line"),
			},
			Token {
				t: Type::Number,
				c: String::from("123"),
			},
			Token {
				t: Type::Operator,
				c: String::from("-"),
			},
		],
	}];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_lines_with_same_indent_level() {
	let tokens = tokenize(
		"single line 123 - \n this is another line",
		get_es_keywords(),
	);

	let expected = vec![
		Line {
			indent: 0,
			tokens: vec![
				Token {
					t: Type::Identifier,
					c: String::from("single"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("line"),
				},
				Token {
					t: Type::Number,
					c: String::from("123"),
				},
				Token {
					t: Type::Operator,
					c: String::from("-"),
				},
			],
		},
		Line {
			indent: 0,
			tokens: vec![
				Token {
					t: Type::Identifier,
					c: String::from("this"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("is"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("another"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("line"),
				},
			],
		},
	];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_opening_keyword_increases_indent_level_for_the_next_lin() {
	let tokens = tokenize("si 1 + 2 entonces\nhaz algo", get_es_keywords());

	let expected = vec![
		Line {
			indent: 0,
			tokens: vec![
				Token {
					t: Type::OpeningKw,
					c: String::from("si"),
				},
				Token {
					t: Type::Number,
					c: String::from("1"),
				},
				Token {
					t: Type::Operator,
					c: String::from("+"),
				},
				Token {
					t: Type::Number,
					c: String::from("2"),
				},
				Token {
					t: Type::RegularKw,
					c: String::from("entonces"),
				},
			],
		},
		Line {
			indent: 1,
			tokens: vec![
				Token {
					t: Type::Identifier,
					c: String::from("haz"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("algo"),
				},
			],
		},
	];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_nested_opening_keywords() {
	let tokens = tokenize("si a \n si b\n1", get_es_keywords());

	let expected = vec![
		Line {
			indent: 0,
			tokens: vec![
				Token {
					t: Type::OpeningKw,
					c: String::from("si"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("a"),
				},
			],
		},
		Line {
			indent: 1,
			tokens: vec![
				Token {
					t: Type::OpeningKw,
					c: String::from("si"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("b"),
				},
			],
		},
		Line {
			indent: 2,
			tokens: vec![Token {
				t: Type::Number,
				c: String::from("1"),
			}],
		},
	];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_closing_keywords() {
	let tokens = tokenize("si a\nfinsi", get_es_keywords());

	let expected = vec![
		Line {
			indent: 0,
			tokens: vec![
				Token {
					t: Type::OpeningKw,
					c: String::from("si"),
				},
				Token {
					t: Type::Identifier,
					c: String::from("a"),
				},
			],
		},
		Line {
			indent: 0,
			tokens: vec![Token {
				t: Type::ClosingKw,
				c: String::from("finsi"),
			}],
		},
	];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}

#[test]
fn test_outdent_cant_be_below_0() {
	let tokens = tokenize("finsi", get_es_keywords());

	let expected = vec![Line {
		indent: 0,
		tokens: vec![Token {
			t: Type::ClosingKw,
			c: String::from("finsi"),
		}],
	}];
	let actual = indent(&tokens);

	assert_eq!(actual, expected);
}
