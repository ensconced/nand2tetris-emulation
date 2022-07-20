use std::rc::Rc;

use super::tokenizer::{
    token_defs, ArithmeticCmdTokenVariant, FunctionCmdTokenVariant, MemoryCmdTokenVariant,
    MemorySegmentTokenVariant, ProgramFlowCmdTokenVariant, TokenKind,
};
use crate::compilers::{utils::{
    parser_utils::{
        maybe_take_command_with_optional_comment_and_whitespace, parse_by_line, PeekableTokens,
    },
    tokenizer::Token,
}, jack_compiler::parser::JackNode};

#[derive(PartialEq, Debug)]
pub enum UnaryArithmeticCommandVariant {
    Neg,
    Not,
}

#[derive(PartialEq, Debug)]
pub enum BinaryArithmeticCommandVariant {
    Add,
    Sub,
    Eq,
    Gt,
    Lt,
    And,
    Or,
}

#[derive(PartialEq, Debug)]
pub enum ArithmeticCommandVariant {
    Unary(UnaryArithmeticCommandVariant),
    Binary(BinaryArithmeticCommandVariant),
}

#[derive(PartialEq, Debug)]
pub enum MemoryCommandVariant {
    Push(MemorySegmentVariant, u16),
    Pop(MemorySegmentVariant, u16),
}

#[derive(PartialEq, Debug)]
pub enum PointerSegmentVariant {
    Argument,
    Local,
    This,
    That,
}

#[derive(PartialEq, Debug)]
pub enum OffsetSegmentVariant {
    Pointer,
    Temp,
}

#[derive(PartialEq, Debug)]
pub enum MemorySegmentVariant {
    PointerSegment(PointerSegmentVariant),
    OffsetSegment(OffsetSegmentVariant),
    Static,
    Constant,
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
    Function(FunctionCommandVariant),
    Flow(FlowCommandVariant),
    Arithmetic(ArithmeticCommandVariant),
    Memory(MemoryCommandVariant),
}

pub struct CommandWithOrigin {
    command: Command,
    origin: Rc<JackNode>,
}

use ArithmeticCommandVariant::*;
use BinaryArithmeticCommandVariant::*;
use Command::*;
use FlowCommandVariant::*;
use FunctionCommandVariant::*;
use MemoryCommandVariant::*;
use MemorySegmentVariant::*;
use OffsetSegmentVariant::*;
use PointerSegmentVariant::*;
use UnaryArithmeticCommandVariant::*;

fn take_arithmetic_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.next() {
        Some(Token {
            kind: TokenKind::ArithmeticCmdToken(kind),
            ..
        }) => match kind {
            ArithmeticCmdTokenVariant::Add => Arithmetic(Binary(Add)),
            ArithmeticCmdTokenVariant::Sub => Arithmetic(Binary(Sub)),
            ArithmeticCmdTokenVariant::Eq => Arithmetic(Binary(Eq)),
            ArithmeticCmdTokenVariant::Gt => Arithmetic(Binary(Gt)),
            ArithmeticCmdTokenVariant::Lt => Arithmetic(Binary(Lt)),
            ArithmeticCmdTokenVariant::And => Arithmetic(Binary(And)),
            ArithmeticCmdTokenVariant::Or => Arithmetic(Binary(Or)),
            ArithmeticCmdTokenVariant::Neg => Arithmetic(Unary(Neg)),
            ArithmeticCmdTokenVariant::Not => Arithmetic(Unary(Not)),
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
            MemorySegmentTokenVariant::Argument => PointerSegment(Argument),
            MemorySegmentTokenVariant::Local => PointerSegment(Local),
            MemorySegmentTokenVariant::That => PointerSegment(That),
            MemorySegmentTokenVariant::This => PointerSegment(This),
            MemorySegmentTokenVariant::Pointer => OffsetSegment(Pointer),
            MemorySegmentTokenVariant::Static => Static,
            MemorySegmentTokenVariant::Temp => OffsetSegment(Temp),
            MemorySegmentTokenVariant::Constant => Constant,
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
            MemoryCmdTokenVariant::Pop => Memory(Pop(segment, index)),
            MemoryCmdTokenVariant::Push => Memory(Push(segment, index)),
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
            FunctionCmdTokenVariant::Return => Function(ReturnFrom),
            FunctionCmdTokenVariant::Define | FunctionCmdTokenVariant::Call => {
                take_whitespace(tokens, line_number);
                let label = take_label(tokens, line_number);
                take_whitespace(tokens, line_number);
                let arg_count = take_number(tokens, line_number);
                if fn_cmd_token == FunctionCmdTokenVariant::Define {
                    Function(Define(label, arg_count))
                } else {
                    Function(Call(label, arg_count))
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
                ProgramFlowCmdTokenVariant::GoTo => Flow(GoTo(label)),
                ProgramFlowCmdTokenVariant::IfGoTo => Flow(IfGoTo(label)),
                ProgramFlowCmdTokenVariant::Label => Flow(Label(label)),
            }
        }
        _ => panic!("expected flow command. line: {}", line_number),
    }
}

fn take_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    if let Some(Token { kind, .. }) = tokens.peek() {
        match kind {
            TokenKind::ArithmeticCmdToken(_) => take_arithmetic_command(tokens, line_number),
            TokenKind::MemoryCmdToken(_) => take_mem_command(tokens, line_number),
            TokenKind::FunctionCmdToken(_) => take_fn_command(tokens, line_number),
            TokenKind::FlowCmdToken(_) => take_flow_command(tokens, line_number),
            _ => panic!("expected command. line: {}", line_number),
        }
    } else {
        panic!("expected command to begin with either arithmetic command, memory command, function command, or flow command. line: {}", line_number);
    }
}

fn parse_line(line_tokens: PeekableTokens<TokenKind>, line_number: usize) -> Option<Command> {
    maybe_take_command_with_optional_comment_and_whitespace(
        line_tokens,
        take_command,
        line_number,
        &TokenKind::Whitespace,
        &TokenKind::Comment,
    )
}

pub fn parse_lines(source: &str) -> impl Iterator<Item = Command> + '_ {
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
            Arithmetic(Binary(Add)),
            Arithmetic(Binary(Sub)),
            Arithmetic(Unary(Neg)),
            Arithmetic(Binary(Eq)),
            Arithmetic(Binary(Gt)),
            Arithmetic(Binary(Lt)),
            Arithmetic(Binary(And)),
            Arithmetic(Binary(Or)),
            Arithmetic(Unary(Not)),
            Memory(Push(PointerSegment(Argument), 1)),
            Memory(Push(PointerSegment(Local), 2)),
            Memory(Push(Static, 3)),
            Memory(Push(Constant, 4)),
            Memory(Pop(PointerSegment(This), 5)),
            Memory(Pop(PointerSegment(That), 6)),
            Memory(Pop(OffsetSegment(Pointer), 7)),
            Memory(Pop(OffsetSegment(Temp), 8)),
            Flow(GoTo("foobar".to_string())),
            Flow(Label("f12.3oo_bA:r".to_string())),
            Flow(IfGoTo("foo:bar".to_string())),
            Function(Define("do_thing".to_string(), 3)),
            Function(Call("do_thing".to_string(), 3)),
            Function(ReturnFrom),
        ];
        assert_eq!(commands, expected)
    }
}
