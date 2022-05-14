use crate::compilers::tokenizer::TokenDef;

#[derive(PartialEq, Debug)]
pub enum ArithmeticCmdTokenVariant {
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
pub enum MemoryCmdTokenVariant {
    Push,
    Pop,
}

#[derive(PartialEq, Debug)]
pub enum MemorySegmentTokenVariant {
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
pub enum ProgramFlowCmdTokenVariant {
    GoTo,
    Label,
    IfGoTo,
}

#[derive(PartialEq, Debug)]
pub enum FunctionCmdTokenVariant {
    Define,
    Call,
    Return,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Comment,
    Whitespace,
    LabelIdentifier(String),
    Number(String),
    FunctionCmdToken(FunctionCmdTokenVariant),
    FlowCmdToken(ProgramFlowCmdTokenVariant),
    ArithmeticCmdToken(ArithmeticCmdTokenVariant),
    MemoryCmdToken(MemoryCmdTokenVariant),
    MemorySegmentToken(MemorySegmentTokenVariant),
}

use ArithmeticCmdTokenVariant::*;
use FunctionCmdTokenVariant::*;
use MemoryCmdTokenVariant::*;
use MemorySegmentTokenVariant::*;
use ProgramFlowCmdTokenVariant::*;
use TokenKind::*;

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| Comment),
        TokenDef::new(r"\s+", |_| Whitespace),
        TokenDef::new(r"[a-zA-Z:_.][0-9a-zA-Z:_.]*", LabelIdentifier),
        TokenDef::new(r"[0-9]+", Number),
        TokenDef::new(r"label", |_| FlowCmdToken(Label)),
        TokenDef::new(r"goto", |_| FlowCmdToken(GoTo)),
        TokenDef::new(r"if-goto", |_| FlowCmdToken(IfGoTo)),
        TokenDef::new(r"function", |_| FunctionCmdToken(Define)),
        TokenDef::new(r"call", |_| FunctionCmdToken(Call)),
        TokenDef::new(r"return", |_| FunctionCmdToken(Return)),
        TokenDef::new(r"add", |_| ArithmeticCmdToken(Add)),
        TokenDef::new(r"sub", |_| ArithmeticCmdToken(Sub)),
        TokenDef::new(r"neg", |_| ArithmeticCmdToken(Neg)),
        TokenDef::new(r"eq", |_| ArithmeticCmdToken(Eq)),
        TokenDef::new(r"gt", |_| ArithmeticCmdToken(Gt)),
        TokenDef::new(r"lt", |_| ArithmeticCmdToken(Lt)),
        TokenDef::new(r"and", |_| ArithmeticCmdToken(And)),
        TokenDef::new(r"or", |_| ArithmeticCmdToken(Or)),
        TokenDef::new(r"not", |_| ArithmeticCmdToken(Not)),
        TokenDef::new(r"push", |_| MemoryCmdToken(Push)),
        TokenDef::new(r"pop", |_| MemoryCmdToken(Pop)),
        TokenDef::new(r"argument", |_| MemorySegmentToken(Argument)),
        TokenDef::new(r"local", |_| MemorySegmentToken(Local)),
        TokenDef::new(r"static", |_| MemorySegmentToken(Static)),
        TokenDef::new(r"constant", |_| MemorySegmentToken(Constant)),
        TokenDef::new(r"this", |_| MemorySegmentToken(This)),
        TokenDef::new(r"that", |_| MemorySegmentToken(That)),
        TokenDef::new(r"pointer", |_| MemorySegmentToken(Pointer)),
        TokenDef::new(r"temp", |_| MemorySegmentToken(Temp)),
    ]
}
