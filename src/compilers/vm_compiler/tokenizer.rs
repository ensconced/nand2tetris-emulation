use crate::compilers::tokenizer::TokenDef;

#[derive(PartialEq, Debug)]
pub enum ArithmeticCmdToken {
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
pub enum MemoryCmdToken {
    Push,
    Pop,
}

#[derive(PartialEq, Debug)]
pub enum MemorySegmentToken {
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
pub enum ProgramFlowCmdToken {
    GoTo,
    Label,
    IfGoTo,
}

#[derive(PartialEq, Debug)]
pub enum FunctionCmdToken {
    Define,
    Call,
    Return,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Comment,
    Whitespace,
    Label(String),
    Number(String),
    FunctionCmdToken(FunctionCmdToken),
    FlowCmdToken(ProgramFlowCmdToken),
    ArithmeticCmdToken(ArithmeticCmdToken),
    MemoryCmdToken(MemoryCmdToken),
    MemorySegmentToken(MemorySegmentToken),
}

use ArithmeticCmdToken::*;
use FunctionCmdToken::*;
use MemoryCmdToken::*;
use MemorySegmentToken::*;
use ProgramFlowCmdToken::*;

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| TokenKind::Comment),
        TokenDef::new(r"\s+", |_| TokenKind::Whitespace),
        TokenDef::new(r"[a-zA-Z:_.][0-9a-zA-Z:_.]*", |src| TokenKind::Label(src)),
        TokenDef::new(r"[0-9]+", |src| TokenKind::Number(src)),
        TokenDef::new(r"label", |_| TokenKind::FlowCmdToken(Label)),
        TokenDef::new(r"goto", |_| TokenKind::FlowCmdToken(GoTo)),
        TokenDef::new(r"if-goto", |_| TokenKind::FlowCmdToken(IfGoTo)),
        TokenDef::new(r"function", |_| TokenKind::FunctionCmdToken(Define)),
        TokenDef::new(r"call", |_| TokenKind::FunctionCmdToken(Call)),
        TokenDef::new(r"return", |_| TokenKind::FunctionCmdToken(Return)),
        TokenDef::new(r"add", |_| TokenKind::ArithmeticCmdToken(Add)),
        TokenDef::new(r"sub", |_| TokenKind::ArithmeticCmdToken(Sub)),
        TokenDef::new(r"neg", |_| TokenKind::ArithmeticCmdToken(Neg)),
        TokenDef::new(r"eq", |_| TokenKind::ArithmeticCmdToken(Eq)),
        TokenDef::new(r"gt", |_| TokenKind::ArithmeticCmdToken(Gt)),
        TokenDef::new(r"lt", |_| TokenKind::ArithmeticCmdToken(Lt)),
        TokenDef::new(r"and", |_| TokenKind::ArithmeticCmdToken(And)),
        TokenDef::new(r"or", |_| TokenKind::ArithmeticCmdToken(Or)),
        TokenDef::new(r"not", |_| TokenKind::ArithmeticCmdToken(Not)),
        TokenDef::new(r"push", |_| TokenKind::MemoryCmdToken(Push)),
        TokenDef::new(r"pop", |_| TokenKind::MemoryCmdToken(Pop)),
        TokenDef::new(r"argument", |_| TokenKind::MemorySegmentToken(Argument)),
        TokenDef::new(r"local", |_| TokenKind::MemorySegmentToken(Local)),
        TokenDef::new(r"static", |_| TokenKind::MemorySegmentToken(Static)),
        TokenDef::new(r"constant", |_| TokenKind::MemorySegmentToken(Constant)),
        TokenDef::new(r"this", |_| TokenKind::MemorySegmentToken(This)),
        TokenDef::new(r"that", |_| TokenKind::MemorySegmentToken(That)),
        TokenDef::new(r"pointer", |_| TokenKind::MemorySegmentToken(Pointer)),
        TokenDef::new(r"temp", |_| TokenKind::MemorySegmentToken(Temp)),
    ]
}
