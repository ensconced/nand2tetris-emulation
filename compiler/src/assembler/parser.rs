use std::fmt::{Display, Formatter, Write};

use serde::Serialize;

use super::tokenizer::{
    token_defs,
    TokenKind::{self, *},
};
use crate::{
    utils::{
        parser_utils::{maybe_take, PeekableTokens},
        tokenizer::{Token, Tokenizer},
    },
    vm_compiler::parser::PointerSegmentVariant,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AValue {
    Numeric(String),
    Symbolic(String),
    Pointer(PointerSegmentVariant),
}

impl Display for AValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let s = match self {
            AValue::Numeric(string) => string,
            AValue::Symbolic(string) => string,
            AValue::Pointer(pointer) => match pointer {
                PointerSegmentVariant::Argument => "arg",
                PointerSegmentVariant::Local => "lcl",
                PointerSegmentVariant::This => "this",
                PointerSegmentVariant::That => "that",
            },
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "String")]
pub enum ASMInstruction {
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

impl From<ASMInstruction> for String {
    fn from(instruction: ASMInstruction) -> Self {
        match instruction {
            ASMInstruction::A(a_value) => format!("@{}", a_value),
            ASMInstruction::C { expr, dest, jump } => {
                let mut s = String::new();
                if let Some(dest_string) = dest {
                    write!(s, "{}=", dest_string).unwrap();
                }
                s.push_str(&expr);
                if let Some(jump_string) = jump {
                    write!(s, ";{}", jump_string).unwrap();
                }
                s
            }
            ASMInstruction::L { identifier } => format!("({})", identifier),
        }
    }
}

fn take_a_value(tokens: &mut PeekableTokens<TokenKind>) -> AValue {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            Number(numeric_string) => AValue::Numeric(numeric_string.to_string()),
            Identifier(identifier_string) => AValue::Symbolic(identifier_string.to_string()),
            _ => panic!("failed to parse a-command as either number or identifier.",),
        },
        _ => panic!("unexpected end of line"),
    }
}

fn take_a_command(tokens: &mut PeekableTokens<TokenKind>) -> ASMInstruction {
    tokens.next(); // pop @
    ASMInstruction::A(take_a_value(tokens))
}

fn take_l_command(tokens: &mut PeekableTokens<TokenKind>) -> ASMInstruction {
    tokens.next(); // pop (
    let token = tokens.next();
    let identifier_string = if let Some(Token {
        kind: Identifier(identifier_string),
        ..
    }) = token
    {
        identifier_string
    } else {
        panic!("failed to parse l-command - expected identifier.",)
    };
    match tokens.next() {
        Some(Token { kind: RParen, .. }) => ASMInstruction::L {
            identifier: identifier_string.to_string(),
        },
        Some(_) => panic!("failed to parse l-command. missing \")\".",),
        None => panic!("failed to parse l-command. unexpected end of line."),
    }
}

fn maybe_take_jump(tokens: &mut PeekableTokens<TokenKind>) -> Option<String> {
    if maybe_take(tokens, &Semicolon).is_some() {
        maybe_take(tokens, &InlineWhitespace);
        if let Some(Token {
            kind: Identifier(identifier_string),
            ..
        }) = tokens.next()
        {
            Some(identifier_string.to_string())
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

fn maybe_take_unary_expression(tokens: &mut PeekableTokens<TokenKind>) -> Option<String> {
    if let Some(Token { kind: Operator(_), .. }) = tokens.peek() {
        if let Some(Token {
            kind: Operator(op_string), ..
        }) = tokens.next()
        {
            let operand = take_single_expression_term(tokens);
            let mut string_copy = op_string.to_string();
            string_copy.push_str(&operand);
            Some(string_copy)
        } else {
            None
        }
    } else {
        None
    }
}

fn take_single_expression_term(tokens: &mut PeekableTokens<TokenKind>) -> String {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            Number(num_string) => num_string.to_string(),
            Identifier(ident_string) => ident_string.to_string(),
            _ => panic!("expected number or identifier as single expression term.",),
        },
        _ => panic!("unexpected end of input."),
    }
}

fn take_binary_or_single_term_expression(tokens: &mut PeekableTokens<TokenKind>) -> String {
    let mut result = take_single_expression_term(tokens);
    if let Some(remainder_string) = maybe_take_unary_expression(tokens) {
        result.push_str(&remainder_string);
    }
    result
}

fn take_unary_expression(tokens: &mut PeekableTokens<TokenKind>) -> String {
    maybe_take_unary_expression(tokens).expect("expected unary expression.")
}

fn take_expression(tokens: &mut PeekableTokens<TokenKind>) -> String {
    match tokens.peek() {
        Some(Token { kind, .. }) => match kind {
            Operator(_) => take_unary_expression(tokens),
            Identifier(_) | Number(_) => take_binary_or_single_term_expression(tokens),
            _ => panic!("unexpected token type while parsing expression",),
        },
        None => panic!("unexpected end of line while parsing expression.",),
    }
}

fn take_c_command(tokens: &mut PeekableTokens<TokenKind>) -> ASMInstruction {
    let dest = maybe_take_destination(tokens);
    let expr = take_expression(tokens);
    maybe_take(tokens, &InlineWhitespace);
    ASMInstruction::C {
        expr,
        dest,
        jump: maybe_take_jump(tokens),
    }
}

fn take_command(tokens: &mut PeekableTokens<TokenKind>) -> ASMInstruction {
    match tokens.peek() {
        Some(Token { kind, .. }) => match kind {
            TokenKind::At => take_a_command(tokens),
            TokenKind::LParen => take_l_command(tokens),
            _ => take_c_command(tokens),
        },
        None => panic!("failed to parse command"),
    }
}

fn take_command_line(tokens: &mut PeekableTokens<TokenKind>) -> ASMInstruction {
    let command = take_command(tokens);
    maybe_take(tokens, &TokenKind::InlineWhitespace);
    maybe_take(tokens, &Comment);
    command
}

pub fn parse(source: &str) -> Vec<ASMInstruction> {
    let tokenizer = Tokenizer::new(token_defs());
    let token_vec = tokenizer.tokenize(source);
    let mut tokens = token_vec.iter().peekable();
    let mut result = Vec::new();

    maybe_take(&mut tokens, &TokenKind::LineBreakingWhitespace);
    maybe_take(&mut tokens, &TokenKind::InlineWhitespace);

    while tokens.peek().is_some() {
        maybe_take(&mut tokens, &TokenKind::InlineWhitespace);
        if maybe_take(&mut tokens, &Comment).is_none() {
            result.push(take_command_line(&mut tokens));
        }
        match tokens.next() {
            Some(Token {
                kind: TokenKind::LineBreakingWhitespace,
                ..
            }) => continue,
            None => break,
            _ => panic!("expected end of line. instead found another token"),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tokenizer::Tokenizer;

    #[test]
    fn test_take_c_command() {
        let tokenizer = Tokenizer::new(token_defs());
        let token_vec = tokenizer.tokenize("M=M+1;JGT");
        let mut tokens = token_vec.iter().peekable();
        let c_command = take_c_command(&mut tokens);
        assert_eq!(
            c_command,
            ASMInstruction::C {
                expr: "M+1".to_string(),
                dest: Some("M".to_string()),
                jump: Some("JGT".to_string())
            }
        );

        let token_vec = tokenizer.tokenize("AMD=A|D;JLT");
        let mut tokens = token_vec.iter().peekable();
        let c_command = take_c_command(&mut tokens);
        assert_eq!(
            c_command,
            ASMInstruction::C {
                expr: "A|D".to_string(),
                dest: Some("AMD".to_string()),
                jump: Some("JLT".to_string())
            }
        );

        let token_vec = tokenizer.tokenize("M+1");
        let mut tokens = token_vec.iter().peekable();
        let c_command = take_c_command(&mut tokens);
        assert_eq!(
            c_command,
            ASMInstruction::C {
                expr: "M+1".to_string(),
                dest: None,
                jump: None
            }
        );

        let token_vec = tokenizer.tokenize("D&M;JGT");
        let mut tokens = token_vec.iter().peekable();
        let c_command = take_c_command(&mut tokens);
        assert_eq!(
            c_command,
            ASMInstruction::C {
                expr: "D&M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let token_vec = tokenizer.tokenize("!M;JGT");
        let mut tokens = token_vec.iter().peekable();
        let c_command = take_c_command(&mut tokens);
        assert_eq!(
            c_command,
            ASMInstruction::C {
                expr: "!M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let token_vec = tokenizer.tokenize("MD=-A");
        let mut chars = token_vec.iter().peekable();
        let c_command = take_c_command(&mut chars);
        assert_eq!(
            c_command,
            ASMInstruction::C {
                expr: "-A".to_string(),
                dest: Some("MD".to_string()),
                jump: None,
            }
        );
    }

    #[test]
    fn test_skip_optional_comment() {
        let tokenizer = Tokenizer::new(token_defs());
        let token_vec = tokenizer.tokenize("// hey there");
        let mut tokens = token_vec.iter().peekable();
        maybe_take(&mut tokens, &Comment);
        let remaining = tokens.next();
        assert_eq!(remaining, None);

        let token_vec = tokenizer.tokenize("not a comment");
        let mut tokens = token_vec.iter().peekable();
        maybe_take(&mut tokens, &Comment);
        let result = tokens.next();
        assert_eq!(result, Some(&Token::new(Identifier("not".to_string()), "not".to_string(), 0),));
    }

    #[test]
    fn test_skip_optional_whitespace() {
        let tokenizer = Tokenizer::new(token_defs());
        let token_vec = tokenizer.tokenize("      hello");
        let mut tokens = token_vec.iter().peekable();
        maybe_take(&mut tokens, &InlineWhitespace);
        let remaining = tokens.next();
        assert_eq!(remaining, Some(&Token::new(Identifier("hello".to_string()), "hello".to_string(), 1)));
    }

    #[test]
    fn test_skip_optional_whitespace_and_comment() {
        let tokenizer = Tokenizer::new(token_defs());
        let token_vec = tokenizer.tokenize("      // this is a comment");
        let mut tokens = token_vec.iter().peekable();
        maybe_take(&mut tokens, &InlineWhitespace);
        maybe_take(&mut tokens, &Comment);
        let remaining = tokens.next();
        assert_eq!(remaining, None);
    }

    #[test]
    fn test_take_a_command() {
        let tokenizer = Tokenizer::new(token_defs());
        let token_vec = tokenizer.tokenize("@1234");
        let mut tokens = token_vec.iter().peekable();
        let a_command = take_a_command(&mut tokens);
        assert_eq!(a_command, ASMInstruction::A(AValue::Numeric("1234".to_string())));

        let token_vec = tokenizer.tokenize("@FOOBAR");
        let mut tokens = token_vec.iter().peekable();
        let a_command = take_a_command(&mut tokens);
        assert_eq!(a_command, ASMInstruction::A(AValue::Symbolic("FOOBAR".to_string())));
    }

    #[test]
    fn test_take_l_command() {
        let tokenizer = Tokenizer::new(token_defs());
        let token_vec = tokenizer.tokenize("(TEST)");
        let mut tokens = token_vec.iter().peekable();
        let a_command = take_l_command(&mut tokens);
        assert_eq!(
            a_command,
            ASMInstruction::L {
                identifier: "TEST".to_string()
            }
        );

        let token_vec = tokenizer.tokenize("(_TEST)");
        let mut tokens = token_vec.iter().peekable();
        let a_command = take_l_command(&mut tokens);
        assert_eq!(
            a_command,
            ASMInstruction::L {
                identifier: "_TEST".to_string()
            }
        );

        let token_vec = tokenizer.tokenize("(T:E$S.T)");
        let mut tokens = token_vec.iter().peekable();
        let a_command = take_l_command(&mut tokens);
        assert_eq!(
            a_command,
            ASMInstruction::L {
                identifier: "T:E$S.T".to_string()
            }
        );
    }

    #[test]
    fn test_parse() {
        let line = "     ";
        let result = parse(line);
        assert_eq!(result.get(0), None);

        let line = "  // hello this is a comment   ";
        let result = parse(line);
        assert_eq!(result.get(0), None);

        let line = "// hello this is a comment";
        let result = parse(line);
        assert_eq!(result.get(0), None);

        let line = "@1234";
        let result = parse(line);
        assert_eq!(result[0], ASMInstruction::A(AValue::Numeric("1234".to_string())));

        let line = "   @1234  // here is a comment  ";
        let result = parse(line);
        assert_eq!(result[0], ASMInstruction::A(AValue::Numeric("1234".to_string())));
    }

    #[test]
    #[should_panic(expected = "expected end of line. instead found another token")]
    fn test_parse_panic() {
        let line = "   @1234 blah blah blah";
        let result = parse(line);
        assert_eq!(result[0], ASMInstruction::A(AValue::Numeric("1234".to_string())));
    }

    #[test]
    fn test_parse_lines() {
        let source = "
        @1234
        AMD=M+1;JGT
            (FOOBAR)
            @FOOBAR
            ";
        let result: Vec<ASMInstruction> = parse(source);
        assert_eq!(
            result,
            vec![
                ASMInstruction::A(AValue::Numeric("1234".to_string())),
                ASMInstruction::C {
                    expr: "M+1".to_string(),
                    dest: Some("AMD".to_string()),
                    jump: Some("JGT".to_string())
                },
                ASMInstruction::L {
                    identifier: "FOOBAR".to_string()
                },
                ASMInstruction::A(AValue::Symbolic("FOOBAR".to_string())),
            ]
        );
    }
}
