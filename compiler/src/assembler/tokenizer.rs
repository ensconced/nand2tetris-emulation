use crate::utils::tokenizer::TokenDef;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Destination(String),
    Identifier(String),
    Number(String),
    Operator(String),
    Comment,
    InlineWhitespace,
    LineBreakingWhitespace,
    At,
    LParen,
    RParen,
    Semicolon,
}

use TokenKind::*;

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| Comment),
        TokenDef::new(r"[AMD]{1,3}=", |src| Destination(src[0..src.len() - 1].to_string())),
        TokenDef::new(r"\s+", |_| LineBreakingWhitespace),
        TokenDef::new(r"[\s&&[^\n]]+", |_| InlineWhitespace),
        TokenDef::new(r"(\||\+|-|&|!)", Operator),
        TokenDef::new(r"[a-zA-Z:$_.][0-9a-zA-Z:$_.]*", Identifier),
        TokenDef::new(r"[0-9]+", Number),
        TokenDef::new(r"@", |_| At),
        TokenDef::new(r"\(", |_| LParen),
        TokenDef::new(r"\)", |_| RParen),
        TokenDef::new(r";", |_| Semicolon),
    ]
}
