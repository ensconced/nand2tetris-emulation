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
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
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
        TokenDef::new(r"<=", |_| Operator(LessThanOrEquals)),
        TokenDef::new(r">", |_| Operator(GreaterThan)),
        TokenDef::new(r">=", |_| Operator(GreaterThanOrEquals)),
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
        Tokenizer::new(token_defs()).tokenize(str)
    }

    #[test]
    fn test_single_line_comment() {
        assert_eq!(
            tokenize("hello there // blah blah blah"),
            vec![
                Token::new(Identifier("hello".to_string()), "hello".to_string(), 0),
                Token::new(Whitespace, " ".to_string(), 1),
                Token::new(Identifier("there".to_string()), "there".to_string(), 2),
                Token::new(Whitespace, " ".to_string(), 3),
                Token::new(SingleLineComment, "// blah blah blah".to_string(), 4),
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
                Token::new(Whitespace, "\n                ".to_string(), 0),
                Token::new(Identifier("hello".to_string()), "hello".to_string(), 1),
                Token::new(Whitespace, " ".to_string(), 2),
                Token::new(Identifier("there".to_string()), "there".to_string(), 3),
                Token::new(Whitespace, "\n                ".to_string(), 4),
                Token::new(MultiLineComment, "/*\n                  here is a big comment\n                  with another line here\n                */".to_string(), 5),
                Token::new(Whitespace, "\n                ".to_string(), 6),
                Token::new(Identifier("and".to_string()), "and".to_string(), 7),
                Token::new(Whitespace, " ".to_string(), 8),
                Token::new(Identifier("more".to_string()), "more".to_string(), 9),
                Token::new(Whitespace, " ".to_string(), 10),
                Token::new(Identifier("identifiers".to_string()), "identifiers".to_string(), 11),
                Token::new(Whitespace, " ".to_string(), 12),
                Token::new(Identifier("here".to_string()), "here".to_string(), 13),
                Token::new(Whitespace, "\n                ".to_string(), 14),
            ]
        );
    }

    #[test]
    fn test_misc_tokens() {
        assert_eq!(
            tokenize("while return { } ( ) [ ] . , ; + * / & | < > = ~ 1234 \"hello\" _aBc123"),
            vec![
                Token::new(Keyword(While), "while".to_string(), 0),
                Token::new(Whitespace, " ".to_string(), 1),
                Token::new(Keyword(Return), "return".to_string(), 2),
                Token::new(Whitespace, " ".to_string(), 3),
                Token::new(LCurly, "{".to_string(), 4),
                Token::new(Whitespace, " ".to_string(), 5),
                Token::new(RCurly, "}".to_string(), 6),
                Token::new(Whitespace, " ".to_string(), 7),
                Token::new(LParen, "(".to_string(), 8),
                Token::new(Whitespace, " ".to_string(), 9),
                Token::new(RParen, ")".to_string(), 10),
                Token::new(Whitespace, " ".to_string(), 11),
                Token::new(LSquareBracket, "[".to_string(), 12),
                Token::new(Whitespace, " ".to_string(), 13),
                Token::new(RSquareBracket, "]".to_string(), 14),
                Token::new(Whitespace, " ".to_string(), 15),
                Token::new(Dot, ".".to_string(), 16),
                Token::new(Whitespace, " ".to_string(), 17),
                Token::new(Comma, ",".to_string(), 18),
                Token::new(Whitespace, " ".to_string(), 19),
                Token::new(Semicolon, ";".to_string(), 20),
                Token::new(Whitespace, " ".to_string(), 21),
                Token::new(Operator(Plus), "+".to_string(), 22),
                Token::new(Whitespace, " ".to_string(), 23),
                Token::new(Operator(Star), "*".to_string(), 24),
                Token::new(Whitespace, " ".to_string(), 25),
                Token::new(Operator(Slash), "/".to_string(), 26),
                Token::new(Whitespace, " ".to_string(), 27),
                Token::new(Operator(Ampersand), "&".to_string(), 28),
                Token::new(Whitespace, " ".to_string(), 29),
                Token::new(Operator(Pipe), "|".to_string(), 30),
                Token::new(Whitespace, " ".to_string(), 31),
                Token::new(Operator(LessThan), "<".to_string(), 32),
                Token::new(Whitespace, " ".to_string(), 33),
                Token::new(Operator(GreaterThan), ">".to_string(), 34),
                Token::new(Whitespace, " ".to_string(), 35),
                Token::new(Operator(Equals), "=".to_string(), 36),
                Token::new(Whitespace, " ".to_string(), 37),
                Token::new(Operator(Tilde), "~".to_string(), 38),
                Token::new(Whitespace, " ".to_string(), 39),
                Token::new(IntegerLiteral("1234".to_string()), "1234".to_string(), 40),
                Token::new(Whitespace, " ".to_string(), 41),
                Token::new(
                    StringLiteral("hello".to_string()),
                    "\"hello\"".to_string(),
                    42
                ),
                Token::new(Whitespace, " ".to_string(), 43),
                Token::new(Identifier("_aBc123".to_string()), "_aBc123".to_string(), 44),
            ]
        );
    }

    #[test]
    fn test_class() {
        assert_eq!(
            tokenize("class foo {}"),
            vec![
                Token::new(Keyword(Class), "class".to_string(), 0),
                Token::new(Whitespace, " ".to_string(), 1),
                Token::new(Identifier("foo".to_string()), "foo".to_string(), 2),
                Token::new(Whitespace, " ".to_string(), 3),
                Token::new(LCurly, "{".to_string(), 4),
                Token::new(RCurly, "}".to_string(), 5),
            ]
        );
    }
}
