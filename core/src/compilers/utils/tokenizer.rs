use regex::{Match, Regex};
use std::fmt::Debug;

pub struct Tokenizer<LangTokenKind> {
    token_defs: Vec<TokenDef<LangTokenKind>>,
}

impl<LangTokenKind: Debug> Tokenizer<LangTokenKind> {
    pub fn new(token_defs: Vec<TokenDef<LangTokenKind>>) -> Self {
        Self { token_defs }
    }

    pub fn tokenize(&self, source: &str) -> Vec<Token<LangTokenKind>> {
        let mut remainder = source.to_string();
        let mut result = Vec::new();
        while let Some(first_token) = get_first_token(&remainder, &self.token_defs) {
            let len = first_token.source.len();
            result.push(first_token);
            remainder = remainder.chars().skip(len).collect();
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<LangTokenKind>
where
    LangTokenKind: 'static,
{
    pub kind: LangTokenKind,
    pub source: String,
}

impl<LangTokenKind> Token<LangTokenKind> {
    #[cfg(test)]
    pub fn new(kind: LangTokenKind, source: String) -> Self {
        Token { kind, source }
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

    fn make_token(&self, match_result: Match) -> Token<LangTokenKind> {
        Token {
            kind: (self.make_token_kind)(match_result.as_str().to_string()),
            source: match_result.as_str().to_string(),
        }
    }

    fn get_token(&self, string: &str) -> Option<Token<LangTokenKind>> {
        self.regex
            .find(string)
            .map(|match_result| self.make_token(match_result))
    }
}

fn get_first_token<LangTokenKind>(
    string: &str,
    token_defs: &[TokenDef<LangTokenKind>],
) -> Option<Token<LangTokenKind>> {
    if string.is_empty() {
        return None;
    }

    // It's not the most efficient way of implementing maximal munch but it
    // does the job.
    let token_alternatives = token_defs
        .iter()
        .filter_map(|matcher| matcher.get_token(string));
    let longest_token = token_alternatives.max_by_key(|token| token.source.len());

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

        let tokens = tokenizer.tokenize("AMD=(@FOO+_bar) ; JMP 1234 // whatever");
        let expected_tokens = vec![
            Token::new(
                TokenKind::Destination("AMD".to_string()),
                "AMD=".to_string(),
            ),
            Token::new(TokenKind::LParen, "(".to_string()),
            Token::new(TokenKind::At, "@".to_string()),
            Token::new(TokenKind::Identifier("FOO".to_string()), "FOO".to_string()),
            Token::new(TokenKind::Operator("+".to_string()), "+".to_string()),
            Token::new(
                TokenKind::Identifier("_bar".to_string()),
                "_bar".to_string(),
            ),
            Token::new(TokenKind::RParen, ")".to_string()),
            Token::new(TokenKind::InlineWhitespace, " ".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
            Token::new(TokenKind::InlineWhitespace, " ".to_string()),
            Token::new(TokenKind::Identifier("JMP".to_string()), "JMP".to_string()),
            Token::new(TokenKind::InlineWhitespace, " ".to_string()),
            Token::new(TokenKind::Number("1234".to_string()), "1234".to_string()),
            Token::new(TokenKind::InlineWhitespace, " ".to_string()),
            Token::new(TokenKind::Comment, "// whatever".to_string()),
        ];
        assert_eq!(tokens, expected_tokens)
    }
}
