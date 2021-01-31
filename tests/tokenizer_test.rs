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
        ]
    }"#
}

#[test]
fn test_empty_string() {
    let expected: Vec<Token> = Vec::new();
    let actual: Vec<Token> = tokenize("", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_single_identifier() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Identifier,
        c: String::from("hola"),
    }];
    let actual: Vec<Token> = tokenize("hola", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_identifiers() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Identifier,
            c: String::from("cosas"),
        },
        Token {
            t: Type::Identifier,
            c: String::from("mundo"),
        },
        Token {
            t: Type::Identifier,
            c: String::from("ayuda"),
        },
    ];
    let actual: Vec<Token> = tokenize("cosas mundo ayuda", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_single_keyword() {
    let expected: Vec<Token> = vec![Token {
        t: Type::OKeyword,
        c: String::from("si"),
    }];
    let actual: Vec<Token> = tokenize("si", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_keywords() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::OKeyword,
            c: String::from("si"),
        },
        Token {
            t: Type::CKeyword,
            c: String::from("finpara"),
        },
        Token {
            t: Type::RKeyword,
            c: String::from("entonces"),
        },
    ];
    let actual: Vec<Token> = tokenize("si finpara entonces", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_combined_identifiers_and_keywords() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Identifier,
            c: String::from("cosas"),
        },
        Token {
            t: Type::RKeyword,
            c: String::from("hasta"),
        },
        Token {
            t: Type::Identifier,
            c: String::from("ayuda"),
        },
    ];
    let actual: Vec<Token> = tokenize("cosas hasta ayuda", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_single_positive_number() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Number,
        c: String::from("12"),
    }];
    let actual: Vec<Token> = tokenize("12", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_positive_numbers() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Number,
            c: String::from("12"),
        },
        Token {
            t: Type::Number,
            c: String::from("23436"),
        },
    ];
    let actual: Vec<Token> = tokenize("12 23436", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_identifier_with_numbers_in_between() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Identifier,
        c: String::from("hola23"),
    }];
    let actual: Vec<Token> = tokenize("hola23", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_number_does_not_integrate_into_identifier_if_at_beginning() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Number,
            c: String::from("54"),
        },
        Token {
            t: Type::Identifier,
            c: String::from("dios"),
        },
    ];
    let actual: Vec<Token> = tokenize("54dios", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_single_newline() {
    let expected: Vec<Token> = vec![Token {
        t: Type::NewLine,
        c: String::new(),
    }];
    let actual: Vec<Token> = tokenize("\n", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_newlines_with_other_tokens() {
    let input_str = "este 1\n3 rtr4\n";

    let expected: Vec<Token> = vec![
        Token {
            t: Type::Identifier,
            c: String::from("este"),
        },
        Token {
            t: Type::Number,
            c: String::from("1"),
        },
        Token {
            t: Type::NewLine,
            c: String::new(),
        },
        Token {
            t: Type::Number,
            c: String::from("3"),
        },
        Token {
            t: Type::Identifier,
            c: String::from("rtr4"),
        },
        Token {
            t: Type::NewLine,
            c: String::new(),
        },
    ];
    let actual: Vec<Token> = tokenize(input_str, get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_delimiters() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Delimiter,
            c: String::from(","),
        },
        Token {
            t: Type::Delimiter,
            c: String::from("."),
        },
        Token {
            t: Type::Delimiter,
            c: String::from("("),
        },
        Token {
            t: Type::Delimiter,
            c: String::from(")"),
        },
    ];
    let actual: Vec<Token> = tokenize(",.()", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_delimiters_limit_keywords() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Identifier,
            c: String::from("hola"),
        },
        Token {
            t: Type::Delimiter,
            c: String::from("."),
        },
        Token {
            t: Type::Number,
            c: String::from("32"),
        },
    ];
    let actual: Vec<Token> = tokenize("hola.32", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_decimal_notated_numbers_in_single_token() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Number,
        c: String::from("0.1"),
    }];
    let actual: Vec<Token> = tokenize("0.1", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_decimal_notated_numebrs() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Number,
            c: String::from("0.1"),
        },
        Token {
            t: Type::Number,
            c: String::from("10.212453"),
        },
        Token {
            t: Type::Number,
            c: String::from("36450.2"),
        },
    ];
    let actual: Vec<Token> = tokenize("0.1 10.212453 36450.2", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_operators() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Operator,
            c: String::from("+"),
        },
        Token {
            t: Type::Operator,
            c: String::from("-"),
        },
        Token {
            t: Type::Operator,
            c: String::from("!"),
        },
    ];
    let actual: Vec<Token> = tokenize("+-!", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_operators_separate_other_tokens() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::RKeyword,
            c: String::from("entonces"),
        },
        Token {
            t: Type::Delimiter,
            c: String::from("."),
        },
        Token {
            t: Type::OKeyword,
            c: String::from("si"),
        },
        Token {
            t: Type::Operator,
            c: String::from("+"),
        },
        Token {
            t: Type::Number,
            c: String::from("24"),
        },
    ];
    let actual: Vec<Token> = tokenize("entonces.si+24", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_negative_number_in_single_token() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Number,
        c: String::from("-1"),
    }];
    let actual: Vec<Token> = tokenize("-1", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_negative_numbers_in_single_token() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Number,
            c: String::from("-24"),
        },
        Token {
            t: Type::Operator,
            c: String::from("+"),
        },
        Token {
            t: Type::Number,
            c: String::from("-54"),
        },
    ];
    let actual: Vec<Token> = tokenize("-24 + -54", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_double_char_operator_in_single_token() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Operator,
        c: String::from("--"),
    }];
    let actual: Vec<Token> = tokenize("--", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_double_char_operators_separated_with_spaces() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Operator,
            c: String::from("++"),
        },
        Token {
            t: Type::Operator,
            c: String::from("--"),
        },
        Token {
            t: Type::Operator,
            c: String::from("!="),
        },
    ];
    let actual: Vec<Token> = tokenize("++ -- !=", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_double_char_operators_no_spaces_in_between() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Operator,
            c: String::from("--"),
        },
        Token {
            t: Type::Operator,
            c: String::from("++"),
        },
        Token {
            t: Type::Operator,
            c: String::from("!="),
        },
    ];
    let actual: Vec<Token> = tokenize("--++!=", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_double_char_operators_separate_from_single_char() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Operator,
            c: String::from("--"),
        },
        Token {
            t: Type::Operator,
            c: String::from("+"),
        },
    ];
    let actual: Vec<Token> = tokenize("--+", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_only_valid_operators_mix_together() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Operator,
            c: String::from("-"),
        },
        Token {
            t: Type::Operator,
            c: String::from("+"),
        },
        Token {
            t: Type::Operator,
            c: String::from("-"),
        },
        Token {
            t: Type::Operator,
            c: String::from("++"),
        },
        Token {
            t: Type::Operator,
            c: String::from("="),
        },
        Token {
            t: Type::Operator,
            c: String::from("!"),
        },
        Token {
            t: Type::Operator,
            c: String::from("<="),
        },
    ];
    let actual: Vec<Token> = tokenize("-+-++=!<=", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn floating_point_negative_number_in_single_token() {
    let expected: Vec<Token> = vec![Token {
        t: Type::Number,
        c: String::from("-0.1"),
    }];
    let actual: Vec<Token> = tokenize("-0.1", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn floating_point_negateive_numbers_in_single_token() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Number,
            c: String::from("-234.34"),
        },
        Token {
            t: Type::Identifier,
            c: String::from("a"),
        },
        Token {
            t: Type::Number,
            c: String::from("-45"),
        },
    ];
    let actual: Vec<Token> = tokenize("-234.34 a -45", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_basic_string() {
    let expected: Vec<Token> = vec![Token {
        t: Type::String,
        c: String::from("\"hola\""),
    }];

    let actual: Vec<Token> = tokenize("\"hola\"", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_string_and_identifiers() {
    let expected: Vec<Token> = vec![
        Token {
            t: Type::Identifier,
            c: String::from("dsf"),
        },
        Token {
            t: Type::String,
            c: String::from("\"this is a string\""),
        },
        Token {
            t: Type::Identifier,
            c: String::from("saf"),
        },
    ];
    let actual: Vec<Token> = tokenize("dsf \"this is a string\" saf", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_strings_include_numbers_and_identifiers() {
    let expected: Vec<Token> = vec![Token {
        t: Type::String,
        c: String::from("\"123.234+34\""),
    }];
    let actual: Vec<Token> = tokenize("\"123.234+34\"", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_multiline_string() {
    let expected: Vec<Token> = vec![Token {
        t: Type::String,
        c: String::from(
            "\"
        something
        \"",
        ),
    }];
    let actual: Vec<Token> = tokenize("\"\n        something\n        \"", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_string_with_other_chars() {
    let expected: Vec<Token> = vec![Token {
        t: Type::String,
        c: String::from("\"%·$/&\""),
    }];
    let actual: Vec<Token> = tokenize("\"%·$/&\"", get_es_keywords());

    assert_eq!(expected, actual);
}

#[test]
fn test_string_with_escaped_quotes() {
    let expected: Vec<Token> = vec![Token {
        t: Type::String,
        c: String::from("\" \\\" \""),
    }];
    let actual: Vec<Token> = tokenize("\" \\\" \"", get_es_keywords());

    assert_eq!(expected, actual);
}
