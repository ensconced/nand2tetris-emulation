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

struct TokenDef {
    regex: Regex,
    make_token_kind: Box<dyn Fn(String) -> TokenKind>,
}

impl TokenDef {
    fn new<T: Fn(String) -> TokenKind + 'static>(regex: &str, make_token_kind: T) -> Self {
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

fn assembly_token_defs() -> Vec<TokenDef> {
    vec![
        TokenDef::new(r"^//.*", |_| TokenKind::Comment),
        TokenDef::new(r"^[AMD]{1,3}=", |src| {
            TokenKind::Destination(src[0..src.len() - 1].to_string())
        }),
        TokenDef::new(r"^\s+", |_| TokenKind::Whitespace),
        // TODO - is this causing a stack overflow?...
        TokenDef::new(r"^(\+|-||&|!)", |src| TokenKind::Operator(src)),
        TokenDef::new(r"^[a-zA-Z:$_.][0-9a-zA-Z:$_.]*", |src| {
            TokenKind::Identifier(src)
        }),
        TokenDef::new(r"^[0-9]+", |src| TokenKind::Number(src)),
        TokenDef::new(r"^@", |_| TokenKind::At),
        TokenDef::new(r"^\(", |_| TokenKind::LParen),
        TokenDef::new(r"^\)", |_| TokenKind::RParen),
        TokenDef::new(r"^;", |_| TokenKind::Semicolon),
    ]
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
        dbg!(string);
        panic!("failed to tokenize");
    }
}

fn tokenize(string: String, token_defs: &Vec<TokenDef>) -> Box<dyn Iterator<Item = Token>> {
    if let Some(first_token) = get_first_token(&string, token_defs) {
        let len = first_token.length;
        let remainder = string.chars().skip(len).collect();
        Box::new(iter::once(first_token).chain(tokenize(remainder, token_defs)))
    } else {
        Box::new(iter::empty())
    }
}

pub fn tokenize_asm_line(line: String) -> Box<dyn Iterator<Item = Token>> {
    let token_defs = assembly_token_defs();
    tokenize(line, &token_defs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        let lexeme_defs = assembly_token_defs();
        let tokens: Vec<Token> = tokenize(
            "AMD=(@FOO+_bar) ; JMP 1234 // whatever".to_string(),
            &lexeme_defs,
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
