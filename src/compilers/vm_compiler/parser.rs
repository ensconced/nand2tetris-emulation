use super::tokenizer::{token_defs, TokenKind};
use crate::compilers::parser_utils::{maybe_take, parse_by_line};
use crate::compilers::tokenizer::Token;
use std::iter::Peekable;

enum ArithmeticCommandVariant {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

enum MemoryCommandVariant {
    Push(MemorySegmentVariant, u16),
    Pop(MemorySegmentVariant, u16),
}

enum MemorySegmentVariant {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

enum FlowCommandVariant {
    GoTo(String),
    Label(String),
    IfGoTo(String),
}

enum FunctionCommandVariant {
    Define(String),
    Call(String),
    ReturnFrom,
}

enum Command {
    FunctionCommand(FunctionCommandVariant),
    FlowCommand(FlowCommandVariant),
    ArithmeticCommand(ArithmeticCommandVariant),
    MemoryCommand(MemoryCommandVariant),
}

fn take_command(
    tokens: &mut Peekable<impl Iterator<Item = Token<TokenKind>>>,
    line_number: usize,
) -> Command {
    todo!()
}

fn parse_line(
    mut line_tokens: Peekable<impl Iterator<Item = Token<TokenKind>>>,
    line_number: usize,
) -> Option<Command> {
    maybe_take(&mut line_tokens, TokenKind::Whitespace);
    maybe_take(&mut line_tokens, TokenKind::Comment);
    if line_tokens.peek().is_none() {
        // There is no command on this line.
        return None;
    }
    let command = take_command(&mut line_tokens, line_number);
    // We could get away with not parsing the rest of the line, but it's good to
    // do, because there could be any kind of syntax errors lurking there...
    maybe_take(&mut line_tokens, TokenKind::Whitespace);
    maybe_take(&mut line_tokens, TokenKind::Comment);
    if let Some(_) = line_tokens.next() {
        panic!(
            "expected end of line. instead found another token. line: {}",
            line_number
        );
    }

    Some(command)
}

pub fn parse_lines<'a>(source: &'a str) -> impl Iterator<Item = Command> + 'a {
    parse_by_line(source, parse_line, token_defs())
}
