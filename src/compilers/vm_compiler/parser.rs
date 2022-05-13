use super::tokenizer::{token_defs, TokenKind};
use crate::compilers::parser_utils::{
    maybe_take_command_with_optional_comment_and_whitespace, parse_by_line,
};
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

// fn parse_line() {
//     todo!()
// }

// pub fn parse_lines<'a>(source: &'a str) -> impl Iterator<Item = Command> + 'a {
//     parse_by_line(source, parse_line, token_defs())
// }
