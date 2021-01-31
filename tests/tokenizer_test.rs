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
