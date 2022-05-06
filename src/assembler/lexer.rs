use lazy_static::lazy_static;
use regex::Regex;
use std::iter;

#[derive(Debug, PartialEq)]
struct Token {
    length: usize,
    kind: TokenKind,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Comment,
    Whitespace,
    Identifier(String),
    Number(String),
    Operator(String),
    At,
    LParen,
    RParen,
    Equals,
    Semicolon,
}

fn get_first_token(string: &str) -> Option<Token> {
    lazy_static! {
        static ref COMMENT: Regex = Regex::new(r"^//.*").expect("failed to compile COMMENT regex");
        static ref WHITESPACE: Regex =
            Regex::new(r"^\s+").expect("failed to compile WHITESPACE regex");
        static ref IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z:$_.][0-9a-zA-Z:$_.]*")
            .expect("failed to compile IDENTIFIER regex");
        static ref NUMBER: Regex = Regex::new(r"^[0-9]+").expect("failed to compile NUMBER regex");
        static ref AT: Regex = Regex::new(r"^@").expect("failed to compile AT regex");
        static ref LPAREN: Regex = Regex::new(r"^(").expect("failed to compile LPAREN regex");
        static ref RPAREN: Regex = Regex::new(r"^)").expect("failed to compile RPAREN regex");
        static ref EQUALS: Regex = Regex::new(r"^=").expect("failed to compile EQUALS regex");
        static ref OPERATOR: Regex =
            Regex::new(r"^[+-|&!]+").expect("failed to compile OPERATOR regex");
        static ref SEMICOLON: Regex = Regex::new(r"^;").expect("failed to compile SEMICOLON regex");
    }
    if string.is_empty() {
        None
    } else if let Some(match_result) = COMMENT.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Comment,
        })
    } else if let Some(match_result) = WHITESPACE.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Whitespace,
        })
    } else if let Some(match_result) = IDENTIFIER.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Identifier(match_string.to_string()),
        })
    } else if let Some(match_result) = NUMBER.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Number(match_string.to_string()),
        })
    } else if let Some(match_result) = OPERATOR.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Operator(match_string.to_string()),
        })
    } else if let Some(match_result) = AT.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::At,
        })
    } else if let Some(match_result) = LPAREN.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::LParen,
        })
    } else if let Some(match_result) = RPAREN.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::RParen,
        })
    } else if let Some(match_result) = EQUALS.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Equals,
        })
    } else if let Some(match_result) = SEMICOLON.find(string) {
        let match_string = match_result.as_str();
        Some(Token {
            length: match_string.len(),
            kind: TokenKind::Semicolon,
        })
    } else {
        panic!("failed to lex token at \"{}\"", string);
    }
}

fn get_tokens(string: String) -> Box<dyn Iterator<Item = Token>> {
    if let Some(first_token) = get_first_token(&string) {
        let len = first_token.length;
        Box::new(iter::once(first_token).chain(get_tokens(string.chars().skip(len).collect())))
    } else {
        Box::new(iter::empty())
    }
}

#[test]
fn test_get_token() {
    let tokens: Vec<Token> = get_tokens("HEY THERE".to_string()).collect();
    assert_eq!(
        tokens,
        vec![
            Token {
                length: 3,
                kind: TokenKind::Identifier("HEY".to_string())
            },
            Token {
                length: 1,
                kind: TokenKind::Whitespace,
            },
            Token {
                length: 5,
                kind: TokenKind::Identifier("THERE".to_string()),
            },
        ]
    )
}
