use crate::compilers::utils::tokenizer::TokenDef;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum OperatorVariant {
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
}

#[derive(Clone, Debug, PartialEq)]
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
    Whitespace,
    SingleLineComment,
    MultiLineComment,
    Operator(OperatorVariant),
}

pub fn token_defs() -> Vec<TokenDef<TokenKind>> {
    use KeywordTokenVariant::*;
    use OperatorVariant::*;
    use TokenKind::*;

    vec![
        TokenDef::new(r"//[^\n]*", |_| SingleLineComment),
        TokenDef::new(r"(?s)/\*.*\*/", |_| MultiLineComment),
        TokenDef::new(r"\s+", |_| Whitespace),
        TokenDef::new(r"\{", |_| LCurly),
        TokenDef::new(r"}", |_| RCurly),
        TokenDef::new(r"\(", |_| LParen),
        TokenDef::new(r"\)", |_| RParen),
        TokenDef::new(r"\[", |_| LSquareBracket),
        TokenDef::new(r"]", |_| RSquareBracket),
        TokenDef::new(r"\.", |_| Dot),
        TokenDef::new(r",", |_| Comma),
        TokenDef::new(r";", |_| Semicolon),
        TokenDef::new(r"\+", |_| Operator(Plus)),
        TokenDef::new(r"-", |_| Operator(Minus)),
        TokenDef::new(r"\*", |_| Operator(Star)),
        TokenDef::new(r"/", |_| Operator(Slash)),
        TokenDef::new(r"&", |_| Operator(Ampersand)),
        TokenDef::new(r"\|", |_| Operator(Pipe)),
        TokenDef::new(r"<", |_| Operator(LessThan)),
        TokenDef::new(r">", |_| Operator(GreaterThan)),
        TokenDef::new(r"=", |_| Operator(Equals)),
        TokenDef::new(r"~", |_| Operator(Tilde)),
        TokenDef::new(r"\d+", IntegerLiteral),
        TokenDef::new("\"[^\"]*\"", |match_str| {
            StringLiteral(match_str[1..match_str.len() - 1].to_string())
        }),
        TokenDef::new(r"[a-zA-Z_][a-zA-Z0-9_]*", Identifier),
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
    ]
}

#[cfg(test)]
mod tests {
    use super::{KeywordTokenVariant::*, OperatorVariant::*, TokenKind::*, *};
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

    #[test]
    fn test_misc_tokens() {
        assert_eq!(
            tokenize("while return { } ( ) [ ] . , ; + * / & | < > = ~ 1234 \"hello\" _aBc123"),
            vec![
                Token::new(5, Keyword(While)),
                Token::new(1, Whitespace),
                Token::new(6, Keyword(Return)),
                Token::new(1, Whitespace),
                Token::new(1, LCurly),
                Token::new(1, Whitespace),
                Token::new(1, RCurly),
                Token::new(1, Whitespace),
                Token::new(1, LParen),
                Token::new(1, Whitespace),
                Token::new(1, RParen),
                Token::new(1, Whitespace),
                Token::new(1, LSquareBracket),
                Token::new(1, Whitespace),
                Token::new(1, RSquareBracket),
                Token::new(1, Whitespace),
                Token::new(1, Dot),
                Token::new(1, Whitespace),
                Token::new(1, Comma),
                Token::new(1, Whitespace),
                Token::new(1, Semicolon),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Plus)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Star)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Slash)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Ampersand)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Pipe)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(LessThan)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(GreaterThan)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Equals)),
                Token::new(1, Whitespace),
                Token::new(1, Operator(Tilde)),
                Token::new(1, Whitespace),
                Token::new(4, IntegerLiteral("1234".to_string())),
                Token::new(1, Whitespace),
                Token::new(7, StringLiteral("hello".to_string())),
                Token::new(1, Whitespace),
                Token::new(7, Identifier("_aBc123".to_string())),
            ]
        );
    }

    #[test]
    fn test_class() {
        assert_eq!(
            tokenize("class foo {}"),
            vec![
                Token::new(5, Keyword(Class)),
                Token::new(1, Whitespace),
                Token::new(3, Identifier("foo".to_string())),
                Token::new(1, Whitespace),
                Token::new(1, LCurly),
                Token::new(1, RCurly),
            ]
        );
    }
}
