use regex::{Match, Regex};
use std::{fmt::Debug, ops::Range};

pub struct Tokenizer<LangTokenKind> {
    token_defs: Vec<TokenDef<LangTokenKind>>,
}

impl<LangTokenKind: Debug> Tokenizer<LangTokenKind> {
    pub fn new(token_defs: Vec<TokenDef<LangTokenKind>>) -> Self {
        Self { token_defs }
    }

    pub fn tokenize(&self, source: &str) -> Box<dyn Iterator<Item = Token<LangTokenKind>>> {
        let mut remainder = source.to_string();
        let mut result = Vec::new();
        while let Some(first_token) = get_first_token(&remainder, source, &self.token_defs) {
            let len = first_token.range.len();
            result.push(first_token);
            remainder = remainder.chars().skip(len).collect();
        }
        Box::new(result.into_iter())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<LangTokenKind>
where
    LangTokenKind: 'static,
{
    pub range: Range<usize>,
    pub kind: LangTokenKind,
}

impl<LangTokenKind> Token<LangTokenKind> {
    #[cfg(test)]
    pub fn new(range: Range<usize>, kind: LangTokenKind) -> Self {
        Token { range, kind }
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

    fn make_token(&self, match_result: Match, stripped_byte_count: usize) -> Token<LangTokenKind> {
        let range = match_result.range();
        let start = range.start + stripped_byte_count;
        let end = range.end + stripped_byte_count;
        Token {
            kind: (self.make_token_kind)(match_result.as_str().to_string()),
            range: start..end,
        }
    }

    fn get_token(&self, string: &str, full_string: &str) -> Option<Token<LangTokenKind>> {
        let stripped_byte_count = full_string.bytes().len() - string.bytes().len();
        self.regex
            .find(string)
            .map(|match_result| self.make_token(match_result, stripped_byte_count))
    }
}

fn get_first_token<LangTokenKind>(
    string: &str,
    full_string: &str,
    token_defs: &[TokenDef<LangTokenKind>],
) -> Option<Token<LangTokenKind>> {
    if string.is_empty() {
        return None;
    }

    // It's not the most efficient way of implementing maximal munch but it
    // does the job.
    let token_alternatives = token_defs
        .iter()
        .filter_map(|matcher| matcher.get_token(string, full_string));
    let longest_token = token_alternatives.max_by_key(|token| token.range.len());

    if longest_token.is_some() {
        longest_token
    } else {
        panic!("failed to tokenize: {}", string);
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
            Token::new(0..4, TokenKind::Destination("AMD".to_string())),
            Token::new(4..5, TokenKind::LParen),
            Token::new(5..6, TokenKind::At),
            Token::new(6..9, TokenKind::Identifier("FOO".to_string())),
            Token::new(9..10, TokenKind::Operator("+".to_string())),
            Token::new(10..14, TokenKind::Identifier("_bar".to_string())),
            Token::new(14..15, TokenKind::RParen),
            Token::new(15..16, TokenKind::Whitespace),
            Token::new(16..17, TokenKind::Semicolon),
            Token::new(17..18, TokenKind::Whitespace),
            Token::new(18..21, TokenKind::Identifier("JMP".to_string())),
            Token::new(21..22, TokenKind::Whitespace),
            Token::new(22..26, TokenKind::Number("1234".to_string())),
            Token::new(26..27, TokenKind::Whitespace),
            Token::new(27..38, TokenKind::Comment),
        ];
        assert_eq!(tokens, expected_tokens)
    }
}
