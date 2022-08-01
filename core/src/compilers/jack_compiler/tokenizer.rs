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
                Token::new(Identifier("hello".to_string()), "hello".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("there".to_string()), "there".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(SingleLineComment, "// blah blah blah".to_string()),
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
                Token::new(Whitespace, "\n                ".to_string()),
                Token::new(Identifier("hello".to_string()), "hello".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("there".to_string()), "there".to_string()),
                Token::new(Whitespace, "\n                ".to_string()),
                Token::new(MultiLineComment, "/*\n                  here is a big comment\n                  with another line here\n                */".to_string()),
                Token::new(Whitespace, "\n                ".to_string()),
                Token::new(Identifier("and".to_string()), "and".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("more".to_string()), "more".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("identifiers".to_string()), "identifiers".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("here".to_string()), "here".to_string()),
                Token::new(Whitespace, "\n                ".to_string()),
            ]
        );
    }

    #[test]
    fn test_misc_tokens() {
        assert_eq!(
            tokenize("while return { } ( ) [ ] . , ; + * / & | < > = ~ 1234 \"hello\" _aBc123"),
            vec![
                Token::new(Keyword(While), "while".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Keyword(Return), "return".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(LCurly, "{".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(RCurly, "}".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(LParen, "(".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(RParen, ")".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(LSquareBracket, "[".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(RSquareBracket, "]".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Dot, ".".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Comma, ",".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Semicolon, ";".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Plus), "+".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Star), "*".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Slash), "/".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Ampersand), "&".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Pipe), "|".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(LessThan), "<".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(GreaterThan), ">".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Equals), "=".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Operator(Tilde), "~".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(IntegerLiteral("1234".to_string()), "1234".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(StringLiteral("hello".to_string()), "\"hello\"".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("_aBc123".to_string()), "_aBc123".to_string()),
            ]
        );
    }

    #[test]
    fn test_class() {
        assert_eq!(
            tokenize("class foo {}"),
            vec![
                Token::new(Keyword(Class), "class".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(Identifier("foo".to_string()), "foo".to_string()),
                Token::new(Whitespace, " ".to_string()),
                Token::new(LCurly, "{".to_string()),
                Token::new(RCurly, "}".to_string()),
            ]
        );
    }
}
