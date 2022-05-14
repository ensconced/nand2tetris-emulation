use crate::compilers::parser_utils::PeekableTokens;

use super::super::parser_utils::{
    maybe_take, maybe_take_command_with_optional_comment_and_whitespace, parse_by_line,
};
use super::super::tokenizer::Token;
use super::tokenizer::{
    token_defs,
    TokenKind::{self, *},
};

#[derive(PartialEq, Debug)]
pub enum AValue {
    Numeric(String),
    Symbolic(String),
}

#[derive(PartialEq, Debug)]
pub enum Command {
    A(AValue),
    C {
        expr: String,
        dest: Option<String>,
        jump: Option<String>,
    },
    L {
        identifier: String,
    },
}

fn take_a_value(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> AValue {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            Number(numeric_string) => AValue::Numeric(numeric_string),
            Identifier(identifier_string) => AValue::Symbolic(identifier_string),
            _ => panic!(
                "failed to parse a-command as either number or identifier. line: {}",
                line_number
            ),
        },
        _ => panic!("unexpected end of line. line: {}", line_number),
    }
}

fn take_a_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    tokens.next(); // pop @
    Command::A(take_a_value(tokens, line_number))
}

fn take_l_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    tokens.next(); // pop (
    let identifier_string = if let Some(Token {
        kind: Identifier(identifier_string),
        ..
    }) = tokens.next()
    {
        identifier_string
    } else {
        panic!(
            "failed to parse l-command - expected identifier. line: {}",
            line_number
        )
    };
    match tokens.next() {
        Some(Token { kind: RParen, .. }) => Command::L {
            identifier: identifier_string,
        },
        Some(_) => panic!(
            "failed to parse l-command. missing \")\". line: {}",
            line_number
        ),
        None => panic!("failed to parse l-command. unexpected end of line."),
    }
}

fn maybe_take_jump(tokens: &mut PeekableTokens<TokenKind>) -> Option<String> {
    if maybe_take(tokens, &Semicolon).is_some() {
        maybe_take(tokens, &Whitespace);
        if let Some(Token {
            kind: Identifier(identifier_string),
            ..
        }) = tokens.next()
        {
            Some(identifier_string)
        } else {
            None
        }
    } else {
        None
    }
}

fn maybe_take_destination(tokens: &mut PeekableTokens<TokenKind>) -> Option<String> {
    if let Some(Token {
        kind: Destination(dest_string),
        ..
    }) = tokens.peek()
    {
        let string = dest_string.to_string();
        tokens.next();
        Some(string)
    } else {
        None
    }
}

fn maybe_take_unary_expression(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> Option<String> {
    if let Some(Token {
        kind: Operator(_), ..
    }) = tokens.peek()
    {
        if let Some(Token {
            kind: Operator(mut op_string),
            ..
        }) = tokens.next()
        {
            let operand = take_single_expression_term(tokens, line_number);
            op_string.push_str(&operand);
            Some(op_string)
        } else {
            None
        }
    } else {
        None
    }
}

fn take_single_expression_term(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> String {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            Number(num_string) => num_string,
            Identifier(ident_string) => ident_string,
            _ => panic!(
                "expected number or identifier as single expression term. line: {}",
                line_number
            ),
        },
        _ => panic!("unexpected end of input. line: {}", line_number),
    }
}

fn take_binary_or_single_term_expression(
    tokens: &mut PeekableTokens<TokenKind>,
    line_number: usize,
) -> String {
    let mut result = take_single_expression_term(tokens, line_number);
    if let Some(remainder_string) = maybe_take_unary_expression(tokens, line_number) {
        result.push_str(&remainder_string);
    }
    result
}

fn take_unary_expression(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    maybe_take_unary_expression(tokens, line_number).expect("expected unary expression.")
}

fn take_expression(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> String {
    match tokens.peek() {
        Some(Token { kind, .. }) => match kind {
            Operator(_) => take_unary_expression(tokens, line_number),
            Identifier(_) | Number(_) => take_binary_or_single_term_expression(tokens, line_number),
            _ => panic!(
                "unexpected token type while parsing expression. line: {}",
                line_number
            ),
        },
        None => panic!(
            "unexpected end of line while parsing expression. line: {}",
            line_number
        ),
    }
}

fn take_c_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    let dest = maybe_take_destination(tokens);
    let expr = take_expression(tokens, line_number);
    maybe_take(tokens, &Whitespace);
    Command::C {
        expr,
        dest,
        jump: maybe_take_jump(tokens),
    }
}

fn take_command(tokens: &mut PeekableTokens<TokenKind>, line_number: usize) -> Command {
    match tokens.peek() {
        Some(Token { kind, .. }) => match kind {
            TokenKind::At => take_a_command(tokens, line_number),
            TokenKind::LParen => take_l_command(tokens, line_number),
            _ => take_c_command(tokens, line_number),
        },
        None => panic!("failed to parse command: line {}", line_number),
    }
}

fn parse_line(line_tokens: PeekableTokens<TokenKind>, line_number: usize) -> Option<Command> {
    maybe_take_command_with_optional_comment_and_whitespace(
        line_tokens,
        take_command,
        line_number,
        &Whitespace,
        &Comment,
    )
}

pub fn parse_lines(source: &str) -> impl Iterator<Item = Command> + '_ {
    parse_by_line(source, parse_line, token_defs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compilers::tokenizer::Tokenizer;

    #[test]
    fn test_take_c_command() {
        let tokenizer = Tokenizer::new(token_defs());
        let mut tokens = tokenizer.tokenize("M=M+1;JGT").peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::C {
                expr: "M+1".to_string(),
                dest: Some("M".to_string()),
                jump: Some("JGT".to_string())
            }
        );

        let mut tokens = tokenizer.tokenize("AMD=A|D;JLT").peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::C {
                expr: "A|D".to_string(),
                dest: Some("AMD".to_string()),
                jump: Some("JLT".to_string())
            }
        );

        let mut tokens = tokenizer.tokenize("M+1").peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::C {
                expr: "M+1".to_string(),
                dest: None,
                jump: None
            }
        );

        let mut tokens = tokenizer.tokenize("D&M;JGT").peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::C {
                expr: "D&M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let mut tokens = tokenizer.tokenize("!M;JGT").peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::C {
                expr: "!M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let mut chars = tokenizer.tokenize("MD=-A").peekable();
        let c_command = take_c_command(&mut chars, 1);
        assert_eq!(
            c_command,
            Command::C {
                expr: "-A".to_string(),
                dest: Some("MD".to_string()),
                jump: None,
            }
        );
    }

    #[test]
    fn test_skip_optional_comment() {
        let tokenizer = Tokenizer::new(token_defs());
        let mut tokens = tokenizer.tokenize("// hey there").peekable();
        maybe_take(&mut tokens, &Comment);
        let remaining = tokens.next();
        assert_eq!(remaining, None);

        let mut tokens = tokenizer.tokenize("not a comment").peekable();
        maybe_take(&mut tokens, &Comment);
        let result = tokens.next();
        assert_eq!(
            result,
            Some(Token {
                kind: Identifier("not".to_string()),
                length: 3
            })
        );
    }

    #[test]
    fn test_skip_optional_whitespace() {
        let tokenizer = Tokenizer::new(token_defs());
        let mut tokens = tokenizer.tokenize("      hello").peekable();
        maybe_take(&mut tokens, &Whitespace);
        let remaining = tokens.next();
        assert_eq!(
            remaining,
            Some(Token {
                kind: Identifier("hello".to_string()),
                length: 5
            })
        );
    }

    #[test]
    fn test_skip_optional_whitespace_and_comment() {
        let tokenizer = Tokenizer::new(token_defs());
        let mut tokens = tokenizer.tokenize("      // this is a comment").peekable();
        maybe_take(&mut tokens, &Whitespace);
        maybe_take(&mut tokens, &Comment);
        let remaining = tokens.next();
        assert_eq!(remaining, None);
    }

    #[test]
    fn test_take_a_command() {
        let tokenizer = Tokenizer::new(token_defs());
        let mut tokens = tokenizer.tokenize("@1234").peekable();
        let a_command = take_a_command(&mut tokens, 1);
        assert_eq!(a_command, Command::A(AValue::Numeric("1234".to_string())));

        let mut tokens = tokenizer.tokenize("@FOOBAR").peekable();
        let a_command = take_a_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::A(AValue::Symbolic("FOOBAR".to_string()))
        );
    }

    #[test]
    fn test_take_l_command() {
        let tokenizer = Tokenizer::new(token_defs());
        let mut tokens = tokenizer.tokenize("(TEST)").peekable();
        let a_command = take_l_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::L {
                identifier: "TEST".to_string()
            }
        );

        let mut tokens = tokenizer.tokenize("(_TEST)").peekable();
        let a_command = take_l_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::L {
                identifier: "_TEST".to_string()
            }
        );

        let mut tokens = tokenizer.tokenize("(T:E$S.T)").peekable();
        let a_command = take_l_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::L {
                identifier: "T:E$S.T".to_string()
            }
        );
    }

    #[test]
    fn test_parse() {
        let tokenizer = Tokenizer::new(token_defs());

        let line = "";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(result, None);

        let line = "     ";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(result, None);

        let line = "  // hello this is a comment   ";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(result, None);

        let line = "// hello this is a comment";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(result, None);

        let line = "@1234";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(
            result,
            Some(Command::A(AValue::Numeric("1234".to_string())))
        );

        let line = "   @1234  // here is a comment  ";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(
            result,
            Some(Command::A(AValue::Numeric("1234".to_string())))
        );
    }

    #[test]
    #[should_panic(expected = "expected end of line. instead found another token")]
    fn test_parse_panic() {
        let tokenizer = Tokenizer::new(token_defs());

        let line = "   @1234 blah blah blah";
        let result = parse_line(tokenizer.tokenize(line).peekable(), 1);
        assert_eq!(
            result,
            Some(Command::A(AValue::Numeric("1234".to_string())))
        );
    }

    #[test]
    fn test_parse_lines() {
        let source = "
        @1234
        AMD=M+1;JGT
            (FOOBAR)
            @FOOBAR
            ";
        let result: Vec<Command> = parse_lines(source).collect();
        assert_eq!(
            result,
            vec![
                Command::A(AValue::Numeric("1234".to_string())),
                Command::C {
                    expr: "M+1".to_string(),
                    dest: Some("AMD".to_string()),
                    jump: Some("JGT".to_string())
                },
                Command::L {
                    identifier: "FOOBAR".to_string()
                },
                Command::A(AValue::Symbolic("FOOBAR".to_string())),
            ]
        );
    }
}
