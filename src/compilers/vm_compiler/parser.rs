use super::tokenizer::{
    token_defs, ArithmeticCmdToken, FunctionCmdToken, MemoryCmdToken, MemorySegmentToken,
    ProgramFlowCmdToken, TokenKind,
};
use crate::compilers::parser_utils::{
    maybe_take, maybe_take_command_with_optional_comment_and_whitespace, parse_by_line,
    PeekableTokens,
};
use crate::compilers::tokenizer::Token;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
enum MemoryCommandVariant {
    Push(MemorySegmentVariant, u16),
    Pop(MemorySegmentVariant, u16),
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
enum FlowCommandVariant {
    GoTo(String),
    Label(String),
    IfGoTo(String),
}

#[derive(PartialEq, Debug)]
enum FunctionCommandVariant {
    Define(String, u16),
    Call(String, u16),
    ReturnFrom,
}

#[derive(PartialEq, Debug)]
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
            FunctionCmdToken::Return => {
                Command::FunctionCommand(FunctionCommandVariant::ReturnFrom)
            }
            FunctionCmdToken::Define | FunctionCmdToken::Call => {
                take_whitespace(tokens, line_number);
                let label = take_label(tokens, line_number);
                take_whitespace(tokens, line_number);
                let arg_count = take_number(tokens, line_number);
                if fn_cmd_token == FunctionCmdToken::Define {
                    Command::FunctionCommand(FunctionCommandVariant::Define(label, arg_count))
                } else {
                    Command::FunctionCommand(FunctionCommandVariant::Call(label, arg_count))
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
                ProgramFlowCmdToken::GoTo => Command::FlowCommand(FlowCommandVariant::GoTo(label)),
                ProgramFlowCmdToken::IfGoTo => {
                    Command::FlowCommand(FlowCommandVariant::IfGoTo(label))
                }
                ProgramFlowCmdToken::Label => {
                    Command::FlowCommand(FlowCommandVariant::Label(label))
                }
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
            Command::ArithmeticCommand(Add),
            Command::ArithmeticCommand(Sub),
            Command::ArithmeticCommand(Neg),
            Command::ArithmeticCommand(Eq),
            Command::ArithmeticCommand(Gt),
            Command::ArithmeticCommand(Lt),
            Command::ArithmeticCommand(And),
            Command::ArithmeticCommand(Or),
            Command::ArithmeticCommand(Not),
            Command::MemoryCommand(MemoryCommandVariant::Push(
                MemorySegmentVariant::Argument,
                1,
            )),
            Command::MemoryCommand(MemoryCommandVariant::Push(MemorySegmentVariant::Local, 2)),
            Command::MemoryCommand(MemoryCommandVariant::Push(MemorySegmentVariant::Static, 3)),
            Command::MemoryCommand(MemoryCommandVariant::Push(
                MemorySegmentVariant::Constant,
                4,
            )),
            Command::MemoryCommand(MemoryCommandVariant::Pop(MemorySegmentVariant::This, 5)),
            Command::MemoryCommand(MemoryCommandVariant::Pop(MemorySegmentVariant::That, 6)),
            Command::MemoryCommand(MemoryCommandVariant::Pop(MemorySegmentVariant::Pointer, 7)),
            Command::MemoryCommand(MemoryCommandVariant::Pop(MemorySegmentVariant::Temp, 8)),
            Command::FlowCommand(FlowCommandVariant::GoTo("foobar".to_string())),
            Command::FlowCommand(FlowCommandVariant::Label("f12.3oo_bA:r".to_string())),
            Command::FlowCommand(FlowCommandVariant::IfGoTo("foo:bar".to_string())),
            Command::FunctionCommand(FunctionCommandVariant::Define("do_thing".to_string(), 3)),
            Command::FunctionCommand(FunctionCommandVariant::Call("do_thing".to_string(), 3)),
            Command::FunctionCommand(FunctionCommandVariant::ReturnFrom),
        ];
        assert_eq!(commands, expected)
    }
}
