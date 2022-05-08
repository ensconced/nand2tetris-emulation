use regex::Regex;
use std::iter;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub length: usize,
    pub kind: TokenKind,
}

impl Token {
    fn new(length: usize, kind: TokenKind) -> Self {
        Token { length, kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Destination(String),
    Identifier(String),
    Number(String),
    Operator(String),
    Comment,
    Whitespace,
    At,
    LParen,
    RParen,
    Semicolon,
}

pub struct TokenDef {
    regex: Regex,
    make_token_kind: Box<dyn Fn(String) -> TokenKind>,
}

impl TokenDef {
    pub fn new<T: Fn(String) -> TokenKind + 'static>(regex: &str, make_token_kind: T) -> Self {
        Self {
            regex: Regex::new(regex).expect("failed to compile regex"),
            make_token_kind: Box::new(make_token_kind),
        }
    }

    fn make_token(&self, string: String) -> Token {
        Token {
            length: string.len(),
            kind: (self.make_token_kind)(string),
        }
    }

    fn get_token(&self, string: &str) -> Option<Token> {
        self.regex
            .find(string)
            .map(|match_result| self.make_token(match_result.as_str().to_string()))
    }
}

fn get_first_token(string: &str, token_defs: &Vec<TokenDef>) -> Option<Token> {
    if string.is_empty() {
        None
    } else if let Some(token) = token_defs
        .iter()
        .find_map(|matcher| matcher.get_token(string))
    {
        Some(token)
    } else {
        panic!("failed to tokenize");
    }
}

pub fn tokenize(string: String, token_defs: &Vec<TokenDef>) -> Box<dyn Iterator<Item = Token>> {
    if let Some(first_token) = get_first_token(&string, token_defs) {
        let len = first_token.length;
        let remainder = string.chars().skip(len).collect();
        Box::new(iter::once(first_token).chain(tokenize(remainder, token_defs)))
    } else {
        Box::new(iter::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::parser::assembly_token_defs;

    #[test]
    fn test_get_token() {
        let token_defs = assembly_token_defs();

        let tokens: Vec<Token> = tokenize(
            "AMD=(@FOO+_bar) ; JMP 1234 // whatever".to_string(),
            &token_defs,
        )
        .collect();
        let expected_tokens = vec![
            Token::new(4, TokenKind::Destination("AMD".to_string())),
            Token::new(1, TokenKind::LParen),
            Token::new(1, TokenKind::At),
            Token::new(3, TokenKind::Identifier("FOO".to_string())),
            Token::new(1, TokenKind::Operator("+".to_string())),
            Token::new(4, TokenKind::Identifier("_bar".to_string())),
            Token::new(1, TokenKind::RParen),
            Token::new(1, TokenKind::Whitespace),
            Token::new(1, TokenKind::Semicolon),
            Token::new(1, TokenKind::Whitespace),
            Token::new(3, TokenKind::Identifier("JMP".to_string())),
            Token::new(1, TokenKind::Whitespace),
            Token::new(4, TokenKind::Number("1234".to_string())),
            Token::new(1, TokenKind::Whitespace),
            Token::new(11, TokenKind::Comment),
        ];
        assert_eq!(tokens, expected_tokens)
    }
}
