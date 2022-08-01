use super::tokenizer::Token;
use std::{iter::Peekable, vec::IntoIter};

pub type PeekableTokens<TokenKind> = Peekable<IntoIter<Token<TokenKind>>>;

pub fn maybe_take<TokenKind>(
    tokens: &mut PeekableTokens<TokenKind>,
    token_kind: &TokenKind,
) -> Option<Token<TokenKind>>
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
