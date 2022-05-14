use super::tokenizer::{Token, TokenDef, Tokenizer};
use std::iter::Peekable;

pub fn maybe_take<TokenKind>(
    tokens: &mut Peekable<impl Iterator<Item = Token<TokenKind>>>,
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

pub type PeekableTokens<TokenKind> = Peekable<Box<dyn Iterator<Item = Token<TokenKind>>>>;
type LineParser<ParsedLine, TokenKind> =
    fn(tokens: PeekableTokens<TokenKind>, line_number: usize) -> Option<ParsedLine>;
type TokenDefs<TokenKind> = Vec<TokenDef<TokenKind>>;

pub fn parse_by_line<ParsedLine, TokenKind>(
    source: &str,
    line_parser: LineParser<ParsedLine, TokenKind>,
    token_defs: TokenDefs<TokenKind>,
) -> impl Iterator<Item = ParsedLine> + '_
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

pub fn maybe_take_command_with_optional_comment_and_whitespace<TokenKind, Command>(
    mut line_tokens: PeekableTokens<TokenKind>,
    take_command: fn(&mut PeekableTokens<TokenKind>, usize) -> Command,
    line_number: usize,
    whitespace: &TokenKind,
    comment: &TokenKind,
) -> Option<Command>
where
    TokenKind: PartialEq + 'static,
{
    maybe_take(&mut line_tokens, whitespace);
    maybe_take(&mut line_tokens, comment);
    line_tokens.peek()?;
    let command = take_command(&mut line_tokens, line_number);
    // We could get away with not parsing the rest of the line, but it's good to
    // do, because there could be any kind of syntax errors lurking there...
    maybe_take(&mut line_tokens, whitespace);
    maybe_take(&mut line_tokens, comment);
    if line_tokens.next().is_some() {
        panic!(
            "expected end of line. instead found another token. line: {}",
            line_number
        );
    }

    Some(command)
}
