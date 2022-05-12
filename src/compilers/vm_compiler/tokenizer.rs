use crate::compilers::tokenizer::TokenDef;

#[derive(PartialEq)]
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

#[derive(PartialEq)]
enum MemoryCommandVariant {
    Push,
    Pop,
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
enum ProgramFlowCommandVariant {
    GoTo,
    Label,
    IfGoTo,
}

#[derive(PartialEq)]
enum FunctionCommandVariant {
    Define,
    Call,
    Return,
}

#[derive(PartialEq)]
pub enum TokenKind {
    Comment,
    Whitespace,
    Label(String),
    Number(String),
    FunctionCommand(FunctionCommandVariant),
    FlowCommand(ProgramFlowCommandVariant),
    ArithmeticCommand(ArithmeticCommandVariant),
    MemoryCommand(MemoryCommandVariant),
    MemorySegment(MemorySegmentVariant),
}

use ArithmeticCommandVariant::*;
use FunctionCommandVariant::*;
use MemoryCommandVariant::*;
use MemorySegmentVariant::*;
use ProgramFlowCommandVariant::*;

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| TokenKind::Comment),
        TokenDef::new(r"\s+", |_| TokenKind::Whitespace),
        TokenDef::new(r"[a-zA-Z:_.][0-9a-zA-Z:_.]*", |src| TokenKind::Label(src)),
        TokenDef::new(r"[0-9]+", |src| TokenKind::Number(src)),
        TokenDef::new(r"label", |_| TokenKind::FlowCommand(Label)),
        TokenDef::new(r"goto", |_| TokenKind::FlowCommand(GoTo)),
        TokenDef::new(r"if-goto", |_| TokenKind::FlowCommand(IfGoTo)),
        TokenDef::new(r"function", |_| TokenKind::FunctionCommand(Define)),
        TokenDef::new(r"call", |_| TokenKind::FunctionCommand(Call)),
        TokenDef::new(r"return", |_| TokenKind::FunctionCommand(Return)),
        TokenDef::new(r"add", |_| TokenKind::ArithmeticCommand(Add)),
        TokenDef::new(r"sub", |_| TokenKind::ArithmeticCommand(Sub)),
        TokenDef::new(r"neg", |_| TokenKind::ArithmeticCommand(Neg)),
        TokenDef::new(r"eq", |_| TokenKind::ArithmeticCommand(Eq)),
        TokenDef::new(r"gt", |_| TokenKind::ArithmeticCommand(Gt)),
        TokenDef::new(r"lt", |_| TokenKind::ArithmeticCommand(Lt)),
        TokenDef::new(r"and", |_| TokenKind::ArithmeticCommand(And)),
        TokenDef::new(r"or", |_| TokenKind::ArithmeticCommand(Or)),
        TokenDef::new(r"not", |_| TokenKind::ArithmeticCommand(Not)),
        TokenDef::new(r"push", |_| TokenKind::MemoryCommand(Push)),
        TokenDef::new(r"pop", |_| TokenKind::MemoryCommand(Pop)),
        TokenDef::new(r"argument", |_| TokenKind::MemorySegment(Argument)),
        TokenDef::new(r"local", |_| TokenKind::MemorySegment(Local)),
        TokenDef::new(r"static", |_| TokenKind::MemorySegment(Static)),
        TokenDef::new(r"constant", |_| TokenKind::MemorySegment(Constant)),
        TokenDef::new(r"this", |_| TokenKind::MemorySegment(This)),
        TokenDef::new(r"that", |_| TokenKind::MemorySegment(That)),
        TokenDef::new(r"pointer", |_| TokenKind::MemorySegment(Pointer)),
        TokenDef::new(r"temp", |_| TokenKind::MemorySegment(Temp)),
    ]
}
