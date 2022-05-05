use lazy_static::lazy_static;
use regex::Regex;

enum Token {
    Identifier(String),
    Number(String),
    At,
    LParen,
    RParen,
    Equals,
    Operator,
    Semicolon,
}

fn lex(line: String) -> Vec<Token> {
    lazy_static! {
        static ref COMMENT: Regex = Regex::new(r"^//.*").expect("failed to compile COMMENT regex");
        static ref WHITESPACE: Regex =
            Regex::new(r"^\s+").expect("failed to compile WHITESPACE regex");
        static ref IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z:$_.][0-9a-zA-Z:$_.]*")
            .expect("failed to compile IDENTIFIER regex");
        static ref NUMBER: Regex = Regex::new(r"^[0-9]+").expect("failed to compile NUMBER regex");
        static ref OPERATOR: Regex =
            Regex::new(r"^[+-|&!]+").expect("failed to compile OPERATOR regex");
    }
    let mut result = Vec::new();
    let line_clone = line.clone();
    let mut chars = line_clone.chars();
    if let Some(lexeme) = COMMENT.find(&line) {
        chars.nth(lexeme.as_str().len() - 1);
    } else if let Some(lexeme) = WHITESPACE.find(&line) {
        chars.nth(lexeme.as_str().len() - 1);
    } else if let Some(lexeme) = IDENTIFIER.find(&line) {
        chars.nth(lexeme.as_str().len() - 1);
        result.push(Token::Identifier(lexeme.as_str().to_string()));
    }
    // TODO etc...
    result
}

#[test]
fn test_foo() {
    assert_eq!(
        COMMENT.find("// hello there").unwrap().as_str(),
        "// hello there"
    );
    assert_eq!(WHITESPACE.find("   ").unwrap().as_str(), "   ");
    assert_eq!(IDENTIFIER.find("hey").unwrap().as_str(), "hey");
    assert_eq!(NUMBER.find("1234").unwrap().as_str(), "1234");
    assert_eq!(OPERATOR.find("+").unwrap().as_str(), "+");
}
