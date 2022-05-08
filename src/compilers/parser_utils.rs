use super::tokenizer::Token;
use std::iter::Peekable;

pub fn skip_optional<TokenKind>(
    tokens: &mut Peekable<impl Iterator<Item = Token<TokenKind>>>,
    token_kind: TokenKind,
) where
    TokenKind: 'static + std::cmp::PartialEq,
{
    if let Some(token) = tokens.peek() {
        if token.kind == token_kind {
            tokens.next();
        }
    }
}

fn parse_line<Command, TokenKind>(
    mut line_tokens: Peekable<impl Iterator<Item = Token<TokenKind>>>,
    line_number: usize,
    whitespace_token_kind: TokenKind,
    comment_token_kind: TokenKind,
) -> Option<Command>
where
    TokenKind: 'static + std::cmp::PartialEq,
{
    skip_optional(&mut line_tokens, whitespace_token_kind);
    skip_optional(&mut line_tokens, AsmTokenKind::Comment);
    if line_tokens.peek().is_none() {
        // There is no command on this line.
        return None;
    }
    let command = take_command(&mut line_tokens, line_number);
    // We could get away with not parsing the rest of the line, but it's good to
    // do, because there could be any kind of syntax errors lurking there...
    skip_optional(&mut line_tokens, whitespace_token_kind);
    skip_optional(&mut line_tokens, AsmTokenKind::Comment);
    if let Some(_) = line_tokens.next() {
        panic!(
            "expected end of line. instead found another token. line: {}",
            line_number
        );
    }

    Some(command)
}

pub fn parse_lines<'a>(source: &'a str) -> impl Iterator<Item = Command> + 'a {
    let lines = source.lines().map(|line| line.to_string());
    let tokenizer = Tokenizer::new(assembly_token_defs());
    lines.enumerate().filter_map(move |(line_idx, line)| {
        let tokens = tokenizer.tokenize(&line).peekable();
        parse_line(tokens, line_idx + 1)
    })
}
