use regex::Regex;
use std::iter;

pub struct Tokenizer<LangTokenKind> {
    token_defs: Vec<TokenDef<LangTokenKind>>,
}

impl<LangTokenKind> Tokenizer<LangTokenKind> {
    pub fn new(token_defs: Vec<TokenDef<LangTokenKind>>) -> Self {
        Self { token_defs }
    }

    pub fn tokenize(&self, source: &str) -> Box<dyn Iterator<Item = Token<LangTokenKind>>> {
        if let Some(first_token) = get_first_token(source, &self.token_defs) {
            let len = first_token.length;
            let remainder: String = source.chars().skip(len).collect();
            Box::new(iter::once(first_token).chain(self.tokenize(&remainder)))
        } else {
            Box::new(iter::empty())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<LangTokenKind>
where
    LangTokenKind: 'static,
{
    pub length: usize,
    pub kind: LangTokenKind,
}

impl<LangTokenKind> Token<LangTokenKind> {
    fn new(length: usize, kind: LangTokenKind) -> Self {
        Token { length, kind }
    }
}

pub struct TokenDef<LangTokenKind> {
    regex: Regex,
    make_token_kind: Box<dyn Fn(String) -> LangTokenKind>,
}

impl<LangTokenKind> TokenDef<LangTokenKind> {
    pub fn new<T: Fn(String) -> LangTokenKind + 'static>(regex: &str, make_token_kind: T) -> Self {
        let full_regex = format!("^{}", regex);
        Self {
            regex: Regex::new(&full_regex).expect("failed to compile regex"),
            make_token_kind: Box::new(make_token_kind),
        }
    }

    fn make_token(&self, string: String) -> Token<LangTokenKind> {
        Token {
            length: string.len(),
            kind: (self.make_token_kind)(string),
        }
    }

    fn get_token(&self, string: &str) -> Option<Token<LangTokenKind>> {
        self.regex
            .find(string)
            .map(|match_result| self.make_token(match_result.as_str().to_string()))
    }
}

fn get_first_token<LangTokenKind>(
    string: &str,
    token_defs: &[TokenDef<LangTokenKind>],
) -> Option<Token<LangTokenKind>> {
    dbg!(string);
    if string.is_empty() {
        return None;
    }

    // It's not the most efficient way of implementing maximal munch but it
    // does the job.
    let token_alternatives = token_defs
        .iter()
        .filter_map(|matcher| matcher.get_token(string));
    let longest_token = token_alternatives.max_by_key(|token| token.length);

    if longest_token.is_some() {
        longest_token
    } else {
        panic!("failed to tokenize");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compilers::assembler::tokenizer::{token_defs, TokenKind};

    #[test]
    fn test_get_token() {
        let tokenizer = Tokenizer::new(token_defs());

        let tokens: Vec<Token<TokenKind>> = tokenizer
            .tokenize("AMD=(@FOO+_bar) ; JMP 1234 // whatever")
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
