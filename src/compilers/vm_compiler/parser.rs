use super::tokenizer::{
    token_defs, ArithmeticCmdTokenVariant, FunctionCmdTokenVariant, MemoryCmdTokenVariant,
    MemorySegmentTokenVariant, ProgramFlowCmdTokenVariant, TokenKind,
};
use crate::compilers::parser_utils::{
    maybe_take_command_with_optional_comment_and_whitespace, parse_by_line, PeekableTokens,
};
use crate::compilers::tokenizer::Token;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum ArithmeticCommandVariant {
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

#[derive(PartialEq, Debug)]
pub enum MemoryCommandVariant {
    Push(MemorySegmentVariant, u16),
    Pop(MemorySegmentVariant, u16),
}

#[derive(PartialEq, Debug)]
pub enum MemorySegmentVariant {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(PartialEq, Debug)]
pub enum FlowCommandVariant {
    GoTo(String),
    Label(String),
    IfGoTo(String),
}

#[derive(PartialEq, Debug)]
pub enum FunctionCommandVariant {
    Define(String, u16),
    Call(String, u16),
    ReturnFrom,
}

#[derive(PartialEq, Debug)]
pub enum Command {
    FunctionCommand(FunctionCommandVariant),
    FlowCommand(FlowCommandVariant),
    ArithmeticCommand(ArithmeticCommandVariant),
    MemoryCommand(MemoryCommandVariant),
}

use ArithmeticCommandVariant::*;
use Command::*;
use FlowCommandVariant::*;
use FunctionCommandVariant::*;
use MemoryCommandVariant::*;
use MemorySegmentVariant::*;

fn take_arithmetic_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::ArithmeticCmdToken(kind),
            ..
        }) => match kind {
            ArithmeticCmdTokenVariant::Add => ArithmeticCommand(Add),
            ArithmeticCmdTokenVariant::Sub => ArithmeticCommand(Sub),
            ArithmeticCmdTokenVariant::Neg => ArithmeticCommand(Neg),
            ArithmeticCmdTokenVariant::Eq => ArithmeticCommand(Eq),
            ArithmeticCmdTokenVariant::Gt => ArithmeticCommand(Gt),
            ArithmeticCmdTokenVariant::Lt => ArithmeticCommand(Lt),
            ArithmeticCmdTokenVariant::And => ArithmeticCommand(And),
            ArithmeticCmdTokenVariant::Or => ArithmeticCommand(Or),
            ArithmeticCmdTokenVariant::Not => ArithmeticCommand(Not),
        },
        _ => panic!("expected arithmetic command token. line: {}", line_number),
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
            MemorySegmentTokenVariant::Argument => Argument,
            MemorySegmentTokenVariant::Constant => Constant,
            MemorySegmentTokenVariant::Local => Local,
            MemorySegmentTokenVariant::Pointer => Pointer,
            MemorySegmentTokenVariant::Static => Static,
            MemorySegmentTokenVariant::Temp => Temp,
            MemorySegmentTokenVariant::That => That,
            MemorySegmentTokenVariant::This => This,
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

fn take_whitespace(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) {
    if let Some(Token {
        kind: TokenKind::Whitespace,
        ..
    }) = tokens.next()
    {
        // all good
    } else {
        panic!("expected whitespace. line: {}", line_number)
    }
}

fn take_mem_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    if let Some(Token {
        kind: TokenKind::MemoryCmdToken(mem_cmd),
        ..
    }) = tokens.next()
    {
        take_whitespace(tokens, line_number);
        let segment = take_mem_segment(tokens, line_number);
        take_whitespace(tokens, line_number);
        let index = take_number(tokens, line_number);
        match mem_cmd {
            MemoryCmdTokenVariant::Pop => MemoryCommand(Pop(segment, index)),
            MemoryCmdTokenVariant::Push => MemoryCommand(Push(segment, index)),
        }
    } else {
        panic!("expected memory command. line: {}", line_number)
    }
}

fn take_label(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::LabelIdentifier(string),
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
            FunctionCmdTokenVariant::Return => FunctionCommand(ReturnFrom),
            FunctionCmdTokenVariant::Define | FunctionCmdTokenVariant::Call => {
                take_whitespace(tokens, line_number);
                let label = take_label(tokens, line_number);
                take_whitespace(tokens, line_number);
                let arg_count = take_number(tokens, line_number);
                if fn_cmd_token == FunctionCmdTokenVariant::Define {
                    FunctionCommand(Define(label, arg_count))
                } else {
                    FunctionCommand(Call(label, arg_count))
                }
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
        }) => {
            take_whitespace(tokens, line_number);
            let label = take_label(tokens, line_number);
            match flow_cmd_token {
                ProgramFlowCmdTokenVariant::GoTo => FlowCommand(GoTo(label)),
                ProgramFlowCmdTokenVariant::IfGoTo => FlowCommand(IfGoTo(label)),
                ProgramFlowCmdTokenVariant::Label => FlowCommand(Label(label)),
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let source = "
            add
            sub
            neg
            eq // here is a comment
            gt
            lt
            and
            // here is a line consisting only of whitespace and a comment
            or
            not // the next line will be whitespace only

            push argument 1
            push local 2
            push static 3 // I'll put some extra spaces and tabs on the next line
            push        constant 4
            pop this 5
            pop that 6
            pop pointer 7
            pop temp 8

            goto foobar
            label f12.3oo_bA:r
            if-goto foo:bar

            function do_thing 3
            call do_thing 3
            return
        ";

        let commands: Vec<Command> = parse_lines(source).collect();
        let expected = vec![
            ArithmeticCommand(Add),
            ArithmeticCommand(Sub),
            ArithmeticCommand(Neg),
            ArithmeticCommand(Eq),
            ArithmeticCommand(Gt),
            ArithmeticCommand(Lt),
            ArithmeticCommand(And),
            ArithmeticCommand(Or),
            ArithmeticCommand(Not),
            MemoryCommand(Push(Argument, 1)),
            MemoryCommand(Push(Local, 2)),
            MemoryCommand(Push(Static, 3)),
            MemoryCommand(Push(Constant, 4)),
            MemoryCommand(Pop(This, 5)),
            MemoryCommand(Pop(That, 6)),
            MemoryCommand(Pop(Pointer, 7)),
            MemoryCommand(Pop(Temp, 8)),
            FlowCommand(GoTo("foobar".to_string())),
            FlowCommand(Label("f12.3oo_bA:r".to_string())),
            FlowCommand(IfGoTo("foo:bar".to_string())),
            FunctionCommand(Define("do_thing".to_string(), 3)),
            FunctionCommand(Call("do_thing".to_string(), 3)),
            FunctionCommand(ReturnFrom),
        ];
        assert_eq!(commands, expected)
    }
}
