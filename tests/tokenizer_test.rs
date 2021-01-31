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
