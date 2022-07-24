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
        let tokenizer = Tokenizer::new(token_defs());
        tokenizer.tokenize(str).collect()
    }

    #[test]
    fn test_single_line_comment() {
        assert_eq!(
            tokenize("hello there // blah blah blah"),
            vec![
                Token::new(0..5, Identifier("hello".to_string())),
                Token::new(5..6, Whitespace),
                Token::new(6..11, Identifier("there".to_string())),
                Token::new(11..12, Whitespace),
                Token::new(12..29, SingleLineComment),
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
                Token::new(0..17, Whitespace),
                Token::new(17..22, Identifier("hello".to_string())),
                Token::new(22..23, Whitespace),
                Token::new(23..28, Identifier("there".to_string())),
                Token::new(28..45, Whitespace),
                Token::new(45..147, MultiLineComment),
                Token::new(147..164, Whitespace),
                Token::new(164..167, Identifier("and".to_string())),
                Token::new(167..168, Whitespace),
                Token::new(168..172, Identifier("more".to_string())),
                Token::new(172..173, Whitespace),
                Token::new(173..184, Identifier("identifiers".to_string())),
                Token::new(184..185, Whitespace),
                Token::new(185..189, Identifier("here".to_string())),
                Token::new(189..206, Whitespace),
            ]
        );
    }

    #[test]
    fn test_misc_tokens() {
        assert_eq!(
            tokenize("while return { } ( ) [ ] . , ; + * / & | < > = ~ 1234 \"hello\" _aBc123"),
            vec![
                Token::new(0..5, Keyword(While)),
                Token::new(5..6, Whitespace),
                Token::new(6..12, Keyword(Return)),
                Token::new(12..13, Whitespace),
                Token::new(13..14, LCurly),
                Token::new(14..15, Whitespace),
                Token::new(15..16, RCurly),
                Token::new(16..17, Whitespace),
                Token::new(17..18, LParen),
                Token::new(18..19, Whitespace),
                Token::new(19..20, RParen),
                Token::new(20..21, Whitespace),
                Token::new(21..22, LSquareBracket),
                Token::new(22..23, Whitespace),
                Token::new(23..24, RSquareBracket),
                Token::new(24..25, Whitespace),
                Token::new(25..26, Dot),
                Token::new(26..27, Whitespace),
                Token::new(27..28, Comma),
                Token::new(28..29, Whitespace),
                Token::new(29..30, Semicolon),
                Token::new(30..31, Whitespace),
                Token::new(31..32, Operator(Plus)),
                Token::new(32..33, Whitespace),
                Token::new(33..34, Operator(Star)),
                Token::new(34..35, Whitespace),
                Token::new(35..36, Operator(Slash)),
                Token::new(36..37, Whitespace),
                Token::new(37..38, Operator(Ampersand)),
                Token::new(38..39, Whitespace),
                Token::new(39..40, Operator(Pipe)),
                Token::new(40..41, Whitespace),
                Token::new(41..42, Operator(LessThan)),
                Token::new(42..43, Whitespace),
                Token::new(43..44, Operator(GreaterThan)),
                Token::new(44..45, Whitespace),
                Token::new(45..46, Operator(Equals)),
                Token::new(46..47, Whitespace),
                Token::new(47..48, Operator(Tilde)),
                Token::new(48..49, Whitespace),
                Token::new(49..53, IntegerLiteral("1234".to_string())),
                Token::new(53..54, Whitespace),
                Token::new(54..61, StringLiteral("hello".to_string())),
                Token::new(61..62, Whitespace),
                Token::new(62..69, Identifier("_aBc123".to_string())),
            ]
        );
    }

    #[test]
    fn test_class() {
        assert_eq!(
            tokenize("class foo {}"),
            vec![
                Token::new(0..5, Keyword(Class)),
                Token::new(5..6, Whitespace),
                Token::new(6..9, Identifier("foo".to_string())),
                Token::new(9..10, Whitespace),
                Token::new(10..11, LCurly),
                Token::new(11..12, RCurly),
            ]
        );
    }
}
