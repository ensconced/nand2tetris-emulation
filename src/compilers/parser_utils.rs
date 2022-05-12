use super::tokenizer::{Token, TokenDef, Tokenizer};
use std::iter::Peekable;

pub fn maybe_take<TokenKind>(
    tokens: &mut Peekable<impl Iterator<Item = Token<TokenKind>>>,
    token_kind: TokenKind,
) -> Option<Token<TokenKind>>
where
    TokenKind: 'static + std::cmp::PartialEq,
{
    if let Some(token) = tokens.peek() {
        if token.kind == token_kind {
            return tokens.next();
        }
    }
    None
}

type PeekableTokens<TokenKind> = Peekable<Box<dyn Iterator<Item = Token<TokenKind>>>>;
type LineParser<ParsedLine, TokenKind> =
    fn(tokens: PeekableTokens<TokenKind>, line_number: usize) -> Option<ParsedLine>;
type TokenDefs<TokenKind> = Vec<TokenDef<TokenKind>>;

pub fn parse_by_line<'a, ParsedLine, TokenKind>(
    source: &'a str,
    line_parser: LineParser<ParsedLine, TokenKind>,
    token_defs: TokenDefs<TokenKind>,
) -> impl Iterator<Item = ParsedLine> + 'a
where
    ParsedLine: 'static,
{
    let lines = source.lines().map(|line| line.to_string());
    let tokenizer = Tokenizer::new(token_defs);
    lines.enumerate().filter_map(move |(line_idx, line)| {
        let tokens = tokenizer.tokenize(&line).peekable();
        line_parser(tokens, line_idx + 1)
    })
}
