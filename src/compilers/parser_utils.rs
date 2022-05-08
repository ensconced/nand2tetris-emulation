use super::tokenizer::Token;
use std::iter::Peekable;

pub fn skip_optional<TokenKind>(
    tokens: &mut Peekable<impl Iterator<Item = Token<TokenKind>>>,
    token_kind: TokenKind,
) -> bool
where
    TokenKind: 'static + std::cmp::PartialEq,
{
    if let Some(token) = tokens.peek() {
        if token.kind == token_kind {
            tokens.next();
            return true;
        }
    }
    false
}
