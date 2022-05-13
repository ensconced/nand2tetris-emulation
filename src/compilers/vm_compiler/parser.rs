use super::tokenizer::{
    token_defs, ArithmeticCmdToken, FunctionCmdToken, MemoryCmdToken, MemorySegmentToken,
    ProgramFlowCmdToken, TokenKind,
};
use crate::compilers::parser_utils::{
    maybe_take_command_with_optional_comment_and_whitespace, parse_by_line, PeekableTokens,
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

use ArithmeticCommandVariant::*;
use Command::*;

fn take_arithmetic_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::ArithmeticCmdToken(kind),
            ..
        }) => match kind {
            ArithmeticCmdToken::Add => ArithmeticCommand(Add),
            ArithmeticCmdToken::Sub => ArithmeticCommand(Sub),
            ArithmeticCmdToken::Neg => ArithmeticCommand(Neg),
            ArithmeticCmdToken::Eq => ArithmeticCommand(Eq),
            ArithmeticCmdToken::Gt => ArithmeticCommand(Gt),
            ArithmeticCmdToken::Lt => ArithmeticCommand(Lt),
            ArithmeticCmdToken::And => ArithmeticCommand(And),
            ArithmeticCmdToken::Or => ArithmeticCommand(Or),
            ArithmeticCmdToken::Not => ArithmeticCommand(Not),
        },
        _ => panic!("expected arithmetic command token"),
    }
}

fn take_mem_segment(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> MemorySegmentVariant {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::MemorySegmentToken(kind),
            ..
        }) => match kind {
            MemorySegmentToken::Argument => MemorySegmentVariant::Argument,
            MemorySegmentToken::Constant => MemorySegmentVariant::Constant,
            MemorySegmentToken::Local => MemorySegmentVariant::Local,
            MemorySegmentToken::Pointer => MemorySegmentVariant::Pointer,
            MemorySegmentToken::Static => MemorySegmentVariant::Static,
            MemorySegmentToken::Temp => MemorySegmentVariant::Temp,
            MemorySegmentToken::That => MemorySegmentVariant::That,
            MemorySegmentToken::This => MemorySegmentVariant::This,
        },
        _ => panic!("expected memory segment token. line: {}", line_number),
    }
}

fn take_number(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> u16 {
    if let Some(Token {
        kind: TokenKind::Number(num_string),
        ..
    }) = tokens.next()
    {
        num_string.parse().expect("failed to parse string as u16")
    } else {
        panic!("expected number. line: {}", line_number)
    }
}

fn take_mem_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    if let Some(Token {
        kind: TokenKind::MemoryCmdToken(mem_cmd),
        ..
    }) = tokens.next()
    {
        let segment = take_mem_segment(tokens, line_number);
        let index = take_number(tokens, line_number);
        match mem_cmd {
            MemoryCmdToken::Pop => {
                Command::MemoryCommand(MemoryCommandVariant::Pop(segment, index))
            }
            MemoryCmdToken::Push => {
                Command::MemoryCommand(MemoryCommandVariant::Push(segment, index))
            }
        }
    } else {
        panic!("expected memory command. line: {}", line_number)
    }
}

fn take_label(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::Label(string),
            ..
        }) => string,
        _ => panic!("expected label. line: {}", line_number),
    }
}

fn take_fn_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::FunctionCmdToken(fn_cmd_token),
            ..
        }) => match fn_cmd_token {
            FunctionCmdToken::Define => Command::FunctionCommand(FunctionCommandVariant::Define(
                take_label(tokens, line_number),
            )),
            FunctionCmdToken::Call => Command::FunctionCommand(FunctionCommandVariant::Call(
                take_label(tokens, line_number),
            )),
            FunctionCmdToken::Return => {
                Command::FunctionCommand(FunctionCommandVariant::ReturnFrom)
            }
        },
        _ => panic!("expected function command. line: {}", line_number),
    }
}

fn take_flow_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::FlowCmdToken(flow_cmd_token),
            ..
        }) => match flow_cmd_token {
            ProgramFlowCmdToken::GoTo => {
                Command::FlowCommand(FlowCommandVariant::GoTo(take_label(tokens, line_number)))
            }
            ProgramFlowCmdToken::IfGoTo => {
                Command::FlowCommand(FlowCommandVariant::IfGoTo(take_label(tokens, line_number)))
            }
            ProgramFlowCmdToken::Label => {
                Command::FlowCommand(FlowCommandVariant::Label(take_label(tokens, line_number)))
            }
        },
        _ => panic!("expected flow command. line: {}", line_number),
    }
}

fn take_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.peek() {
        Some(token) => match token {
            Token { kind, .. } => match kind {
                TokenKind::ArithmeticCmdToken(_) => {
                    take_arithmetic_command(tokens, line_number)
                }
                TokenKind::MemoryCmdToken(_) => take_mem_command(tokens, line_number),
                TokenKind::FunctionCmdToken(_) => take_fn_command(tokens, line_number),
                TokenKind::FlowCmdToken(_) => take_flow_command(tokens, line_number),
                _ => panic!(),
            },
        },
        _ => panic!("expected command to begin with either arithmetic command, memory command, function command, or flow command. line: {}", line_number),
    }
}

fn parse_line(
    line_tokens: Peekable<Box<dyn Iterator<Item = Token<TokenKind>>>>,
    line_number: usize,
) -> Option<Command> {
    maybe_take_command_with_optional_comment_and_whitespace(
        line_tokens,
        take_command,
        line_number,
        &TokenKind::Whitespace,
        &TokenKind::Comment,
    )
}

pub fn parse_lines<'a>(source: &'a str) -> impl Iterator<Item = Command> + 'a {
    parse_by_line(source, parse_line, token_defs())
}
