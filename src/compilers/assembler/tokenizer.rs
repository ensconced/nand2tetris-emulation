use super::super::tokenizer::TokenDef;

#[derive(Debug, PartialEq)]
pub enum AsmTokenKind {
    Destination(String),
    Identifier(String),
    Number(String),
    Operator(String),
    Comment,
    Whitespace,
    At,
    LParen,
    RParen,
    Semicolon,
}

pub fn assembly_token_defs() -> Vec<TokenDef<AsmTokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| AsmTokenKind::Comment),
        TokenDef::new(r"[AMD]{1,3}=", |src| {
            AsmTokenKind::Destination(src[0..src.len() - 1].to_string())
        }),
        TokenDef::new(r"\s+", |_| AsmTokenKind::Whitespace),
        TokenDef::new(r"(\||\+|-|&|!)", |src| AsmTokenKind::Operator(src)),
        TokenDef::new(r"[a-zA-Z:$_.][0-9a-zA-Z:$_.]*", |src| {
            AsmTokenKind::Identifier(src)
        }),
        TokenDef::new(r"[0-9]+", |src| AsmTokenKind::Number(src)),
        TokenDef::new(r"@", |_| AsmTokenKind::At),
        TokenDef::new(r"\(", |_| AsmTokenKind::LParen),
        TokenDef::new(r"\)", |_| AsmTokenKind::RParen),
        TokenDef::new(r";", |_| AsmTokenKind::Semicolon),
    ]
}
