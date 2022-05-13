use super::super::tokenizer::TokenDef;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    vec![
        TokenDef::new(r"//.*", |_| TokenKind::Comment),
        TokenDef::new(r"[AMD]{1,3}=", |src| {
            TokenKind::Destination(src[0..src.len() - 1].to_string())
        }),
        TokenDef::new(r"\s+", |_| TokenKind::Whitespace),
        TokenDef::new(r"(\||\+|-|&|!)", |src| TokenKind::Operator(src)),
        TokenDef::new(r"[a-zA-Z:$_.][0-9a-zA-Z:$_.]*", |src| {
            TokenKind::Identifier(src)
        }),
        TokenDef::new(r"[0-9]+", |src| TokenKind::Number(src)),
        TokenDef::new(r"@", |_| TokenKind::At),
        TokenDef::new(r"\(", |_| TokenKind::LParen),
        TokenDef::new(r"\)", |_| TokenKind::RParen),
        TokenDef::new(r";", |_| TokenKind::Semicolon),
    ]
}
