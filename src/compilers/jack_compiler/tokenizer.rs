use crate::compilers::utils::tokenizer::TokenDef;

#[derive(Debug, PartialEq)]
pub enum KeywordTokenVariant {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(KeywordTokenVariant),
    IntegerLiteral(String),
    StringLiteral(String),
    Identifier(String),
    LCurly,
    RCurly,
    LParen,
    RParen,
    LSquareBracket,
    RSquareBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Ampersand,
    Pipe,
    LessThan,
    GreaterThan,
    Equals,
    Tilde,
    Whitespace,
    SingleLineComment,
    MultiLineComment,
}

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    use KeywordTokenVariant::*;
    use TokenKind::*;

    vec![
        TokenDef::new(r"//[^\n]*", |_| SingleLineComment),
        TokenDef::new(r"(?s)/\*.*\*/", |_| MultiLineComment),
        TokenDef::new(r"\s+", |_| Whitespace),
        TokenDef::new(r"class", |_| Keyword(Class)),
        TokenDef::new(r"constructor", |_| Keyword(Constructor)),
        TokenDef::new(r"function", |_| Keyword(Function)),
        TokenDef::new(r"method", |_| Keyword(Method)),
        TokenDef::new(r"field", |_| Keyword(Field)),
        TokenDef::new(r"static", |_| Keyword(Static)),
        TokenDef::new(r"var", |_| Keyword(Var)),
        TokenDef::new(r"int", |_| Keyword(Int)),
        TokenDef::new(r"char", |_| Keyword(Char)),
        TokenDef::new(r"boolean", |_| Keyword(Boolean)),
        TokenDef::new(r"void", |_| Keyword(Void)),
        TokenDef::new(r"true", |_| Keyword(True)),
        TokenDef::new(r"false", |_| Keyword(False)),
        TokenDef::new(r"null", |_| Keyword(Null)),
        TokenDef::new(r"this", |_| Keyword(This)),
        TokenDef::new(r"let", |_| Keyword(Let)),
        TokenDef::new(r"do", |_| Keyword(Do)),
        TokenDef::new(r"if", |_| Keyword(If)),
        TokenDef::new(r"else", |_| Keyword(Else)),
        TokenDef::new(r"while", |_| Keyword(While)),
        TokenDef::new(r"return", |_| Keyword(Return)),
        TokenDef::new(r"\{", |_| LCurly),
        TokenDef::new(r"}", |_| RCurly),
        TokenDef::new(r"\(", |_| LParen),
        TokenDef::new(r"\)", |_| RParen),
        TokenDef::new(r"\[", |_| LSquareBracket),
        TokenDef::new(r"]", |_| RSquareBracket),
        TokenDef::new(r"\.", |_| Dot),
        TokenDef::new(r",", |_| Comma),
        TokenDef::new(r";", |_| Semicolon),
        TokenDef::new(r"+", |_| Plus),
        TokenDef::new(r"-", |_| Minus),
        TokenDef::new(r"\*", |_| Star),
        TokenDef::new(r"/", |_| Slash),
        TokenDef::new(r"&", |_| Ampersand),
        TokenDef::new(r"\|", |_| Pipe),
        TokenDef::new(r"<", |_| LessThan),
        TokenDef::new(r">", |_| GreaterThan),
        TokenDef::new(r"=", |_| Equals),
        TokenDef::new(r"~", |_| Tilde),
        TokenDef::new(r"\d+", IntegerLiteral),
        TokenDef::new("\"[^\"\n]\"", StringLiteral),
        TokenDef::new(r"[a-zA-Z_][a-zA-Z0-9_]*", Identifier),
    ]
}

#[cfg(test)]
mod tests {
    use super::{KeywordTokenVariant::*, TokenKind::*, *};
    use crate::compilers::utils::tokenizer::{Token, Tokenizer};

    fn tokenize(str: &str) -> Vec<Token<TokenKind>> {
        let tokenizer = Tokenizer::new(token_defs());
        tokenizer.tokenize(str).collect()
    }

    #[test]
    fn test_single_line_comment() {
        assert_eq!(
            tokenize("hello there // blah blah blah"),
            vec![
                Token::new(5, Identifier("hello".to_string())),
                Token::new(1, Whitespace),
                Token::new(5, Identifier("there".to_string())),
                Token::new(1, Whitespace),
                Token::new(17, SingleLineComment),
            ]
        );
    }

    #[test]
    fn test_multi_line_comment() {
        assert_eq!(
            tokenize(
                "
                hello there
                /*
                  here is a big comment
                  with another line here
                */
                and more identifiers here
                "
            ),
            vec![
                Token::new(17, Whitespace),
                Token::new(5, Identifier("hello".to_string())),
                Token::new(1, Whitespace),
                Token::new(5, Identifier("there".to_string())),
                Token::new(17, Whitespace),
                Token::new(102, MultiLineComment),
                Token::new(17, Whitespace),
                Token::new(3, Identifier("and".to_string())),
                Token::new(1, Whitespace),
                Token::new(4, Identifier("more".to_string())),
                Token::new(1, Whitespace),
                Token::new(11, Identifier("identifiers".to_string())),
                Token::new(1, Whitespace),
                Token::new(4, Identifier("here".to_string())),
                Token::new(17, Whitespace),
            ]
        );
    }
}
