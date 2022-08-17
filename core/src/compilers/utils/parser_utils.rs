use super::tokenizer::Token;
use std::{iter::Peekable, slice::Iter};

pub type PeekableTokens<'a, TokenKind> = Peekable<Iter<'a, Token<TokenKind>>>;

pub fn maybe_take<'a, TokenKind>(tokens: &'a mut PeekableTokens<TokenKind>, token_kind: &TokenKind) -> Option<&'a Token<TokenKind>>
where
    TokenKind: 'static + std::cmp::PartialEq,
{
    if let Some(token) = tokens.peek() {
        if token.kind == *token_kind {
            return tokens.next();
        }
    }
    None
}
