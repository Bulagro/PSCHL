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
