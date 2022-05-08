use crate::tokenizer::{tokenize, Token, TokenDef};

enum ArithmeticCommand {
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

enum MemoryCommand {
    Push,
    Pop,
}

enum MemorySegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

enum Keyword {
    Label,
    GoTo,
    IfGoTo,
    Function,
    Call,
    Return,
    ArithmeticCommand(ArithmeticCommand),
    MemoryCommand(MemoryCommand),
    MemorySegment(MemorySegment),
}

enum VMTokenKind {
    Comment,
    Whitespace,
    Label(String),
    Number(String),
    Keyword(Keyword),
}

fn vm_token_defs() -> Vec<TokenDef<VMTokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| VMTokenKind::Comment),
        TokenDef::new(r"\s+", |_| VMTokenKind::Whitespace),
        TokenDef::new(r"[a-zA-Z:_.][0-9a-zA-Z:_.]*", |src| VMTokenKind::Label(src)),
        TokenDef::new(r"[0-9]+", |src| VMTokenKind::Number(src)),
        TokenDef::new(r"label", |_| VMTokenKind::Keyword(Keyword::Label)),
        TokenDef::new(r"goto", |_| VMTokenKind::Keyword(Keyword::GoTo)),
        TokenDef::new(r"if_goto", |_| VMTokenKind::Keyword(Keyword::IfGoTo)),
        TokenDef::new(r"function", |_| VMTokenKind::Keyword(Keyword::Function)),
        TokenDef::new(r"call", |_| VMTokenKind::Keyword(Keyword::Call)),
        TokenDef::new(r"return", |_| VMTokenKind::Keyword(Keyword::Return)),
        TokenDef::new(r"add", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Add))
        }),
        TokenDef::new(r"sub", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Sub))
        }),
        TokenDef::new(r"neg", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Neg))
        }),
        TokenDef::new(r"eq", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Eq))
        }),
        TokenDef::new(r"gt", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Gt))
        }),
        TokenDef::new(r"lt", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Lt))
        }),
        TokenDef::new(r"and", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::And))
        }),
        TokenDef::new(r"or", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Or))
        }),
        TokenDef::new(r"not", |_| {
            VMTokenKind::Keyword(Keyword::ArithmeticCommand(ArithmeticCommand::Not))
        }),
        TokenDef::new(r"push", |_| {
            VMTokenKind::Keyword(Keyword::MemoryCommand(MemoryCommand::Push))
        }),
        TokenDef::new(r"pop", |_| {
            VMTokenKind::Keyword(Keyword::MemoryCommand(MemoryCommand::Pop))
        }),
        TokenDef::new(r"argument", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::Argument))
        }),
        TokenDef::new(r"local", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::Local))
        }),
        TokenDef::new(r"static", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::Static))
        }),
        TokenDef::new(r"constant", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::Constant))
        }),
        TokenDef::new(r"this", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::This))
        }),
        TokenDef::new(r"that", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::That))
        }),
        TokenDef::new(r"pointer", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::Pointer))
        }),
        TokenDef::new(r"temp", |_| {
            VMTokenKind::Keyword(Keyword::MemorySegment(MemorySegment::Temp))
        }),
    ]
}
