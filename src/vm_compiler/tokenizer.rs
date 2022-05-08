use crate::tokenizer::TokenDef;

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
    Push,
    Pop,
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

enum ProgramFlowCommandVariant {
    GoTo,
    Label,
    IfGoTo,
}

enum FunctionCommandVariant {
    Define,
    Call,
    Return,
}

enum VMTokenKind {
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

fn vm_token_defs() -> Vec<TokenDef<VMTokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| VMTokenKind::Comment),
        TokenDef::new(r"\s+", |_| VMTokenKind::Whitespace),
        TokenDef::new(r"[a-zA-Z:_.][0-9a-zA-Z:_.]*", |src| VMTokenKind::Label(src)),
        TokenDef::new(r"[0-9]+", |src| VMTokenKind::Number(src)),
        TokenDef::new(r"label", |_| VMTokenKind::FlowCommand(Label)),
        TokenDef::new(r"goto", |_| VMTokenKind::FlowCommand(GoTo)),
        TokenDef::new(r"if-goto", |_| VMTokenKind::FlowCommand(IfGoTo)),
        TokenDef::new(r"function", |_| VMTokenKind::FunctionCommand(Define)),
        TokenDef::new(r"call", |_| VMTokenKind::FunctionCommand(Call)),
        TokenDef::new(r"return", |_| VMTokenKind::FunctionCommand(Return)),
        TokenDef::new(r"add", |_| VMTokenKind::ArithmeticCommand(Add)),
        TokenDef::new(r"sub", |_| VMTokenKind::ArithmeticCommand(Sub)),
        TokenDef::new(r"neg", |_| VMTokenKind::ArithmeticCommand(Neg)),
        TokenDef::new(r"eq", |_| VMTokenKind::ArithmeticCommand(Eq)),
        TokenDef::new(r"gt", |_| VMTokenKind::ArithmeticCommand(Gt)),
        TokenDef::new(r"lt", |_| VMTokenKind::ArithmeticCommand(Lt)),
        TokenDef::new(r"and", |_| VMTokenKind::ArithmeticCommand(And)),
        TokenDef::new(r"or", |_| VMTokenKind::ArithmeticCommand(Or)),
        TokenDef::new(r"not", |_| VMTokenKind::ArithmeticCommand(Not)),
        TokenDef::new(r"push", |_| VMTokenKind::MemoryCommand(Push)),
        TokenDef::new(r"pop", |_| VMTokenKind::MemoryCommand(Pop)),
        TokenDef::new(r"argument", |_| VMTokenKind::MemorySegment(Argument)),
        TokenDef::new(r"local", |_| VMTokenKind::MemorySegment(Local)),
        TokenDef::new(r"static", |_| VMTokenKind::MemorySegment(Static)),
        TokenDef::new(r"constant", |_| VMTokenKind::MemorySegment(Constant)),
        TokenDef::new(r"this", |_| VMTokenKind::MemorySegment(This)),
        TokenDef::new(r"that", |_| VMTokenKind::MemorySegment(That)),
        TokenDef::new(r"pointer", |_| VMTokenKind::MemorySegment(Pointer)),
        TokenDef::new(r"temp", |_| VMTokenKind::MemorySegment(Temp)),
    ]
}
