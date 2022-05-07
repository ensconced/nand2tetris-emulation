use super::tokenizer::{tokenize_asm_line, Token, TokenKind};
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum AValue {
    Numeric(String),
    Symbolic(String),
}

#[derive(PartialEq, Debug)]
pub enum Command {
    ACommand(AValue),
    CCommand {
        expr: String,
        dest: Option<String>,
        jump: Option<String>,
    },
    LCommand {
        identifier: String,
    },
}

fn skip_optional_comment(tokens: &mut Peekable<impl Iterator<Item = Token>>) {
    if let Some(Token {
        kind: TokenKind::Comment,
        ..
    }) = tokens.peek()
    {
        tokens.next();
    }
}

fn skip_optional_whitespace(tokens: &mut Peekable<impl Iterator<Item = Token>>) {
    if let Some(Token {
        kind: TokenKind::Whitespace,
        ..
    }) = tokens.peek()
    {
        tokens.next();
    }
}

fn take_a_value(tokens: &mut Peekable<impl Iterator<Item = Token>>, line_number: usize) -> AValue {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            TokenKind::Number(numeric_string) => AValue::Numeric(numeric_string.to_string()),
            TokenKind::Identifier(identifier_string) => {
                AValue::Symbolic(identifier_string.to_string())
            }
            _ => panic!(
                "failed to parse a-command as either number or identifier. line: {}",
                line_number
            ),
        },
        _ => panic!("unexpected end of line. line: {}", line_number),
    }
}

fn take_a_command(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> Command {
    tokens.next(); // @
    let a_value = take_a_value(tokens, line_number);
    Command::ACommand(a_value)
}

fn take_l_command(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> Command {
    tokens.next(); // (
    let identifier_string = if let Some(Token {
        kind: TokenKind::Identifier(identifier_string),
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
        Some(Token {
            kind: TokenKind::RParen,
            ..
        }) => Command::LCommand {
            identifier: identifier_string,
        },
        Some(_) => panic!(
            "failed to parse l-command. missing \")\". line: {}",
            line_number
        ),
        None => panic!("failed to parse l-command. unexpected end of line."),
    }
}

fn take_optional_jump(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<String> {
    // TODO - validation should be done later, to be more consistent with other identifiers
    // let valid_jumps: Vec<String> = vec!["JGT", "JEQ", "JGE", "JLT", "JNE", "JLE", "JMP"]
    //     .into_iter()
    //     .map(String::from)
    //     .collect();
    if let Some(Token {
        kind: TokenKind::Semicolon,
        ..
    }) = tokens.peek()
    {
        tokens.next(); // pop semicolon
        skip_optional_whitespace(tokens);
        if let Some(Token {
            kind: TokenKind::Identifier(identifier_string),
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

fn take_optional_destination(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<String> {
    if let Some(Token {
        kind: TokenKind::Destination(dest_string),
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

fn take_optional_unary_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> Option<String> {
    if let Some(Token {
        kind: TokenKind::Operator(_),
        ..
    }) = tokens.peek()
    {
        if let Some(Token {
            kind: TokenKind::Operator(mut op_string),
            ..
        }) = tokens.next()
        {
            let operand = take_single_expression_term(tokens, line_number);
            op_string.extend(operand.chars());
            Some(op_string)
        } else {
            None
        }
    } else {
        None
    }
}

fn take_single_expression_term(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> String {
    match tokens.next() {
        Some(Token { kind, .. }) => match kind {
            TokenKind::Number(num_string) => num_string,
            TokenKind::Identifier(ident_string) => ident_string,
            _ => panic!(
                "expected number or identifier as single expression term. line: {}",
                line_number
            ),
        },
        _ => panic!("unexpected end of input. line: {}", line_number),
    }
}

fn take_binary_or_single_term_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> String {
    let mut result = take_single_expression_term(tokens, line_number);
    if let Some(remainder_string) = take_optional_unary_expression(tokens, line_number) {
        result.extend(remainder_string.chars());
    }
    result
}

fn take_unary_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> String {
    take_optional_unary_expression(tokens, line_number).expect("expected unary expression.")
}

fn take_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> String {
    match tokens.peek() {
        Some(Token { kind, .. }) => match kind {
            TokenKind::Operator(_) => take_unary_expression(tokens, line_number),
            TokenKind::Identifier(_) | TokenKind::Number(_) => {
                take_binary_or_single_term_expression(tokens, line_number)
            }
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

fn take_c_command(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    line_number: usize,
) -> Command {
    let dest = take_optional_destination(tokens);
    let expr = take_expression(tokens, line_number);
    skip_optional_whitespace(tokens);
    Command::CCommand {
        expr,
        dest,
        jump: take_optional_jump(tokens),
    }
}

fn take_command(tokens: &mut Peekable<impl Iterator<Item = Token>>, line_number: usize) -> Command {
    match tokens.peek() {
        Some(Token { kind, .. }) => match kind {
            TokenKind::At => take_a_command(tokens, line_number),
            TokenKind::LParen => take_l_command(tokens, line_number),
            _ => take_c_command(tokens, line_number),
        },
        None => panic!("failed to parse command: line {}", line_number),
    }
}

fn parse_line(line: &str, line_number: usize) -> Option<Command> {
    let mut tokens = tokenize_asm_line(line.to_string()).peekable();
    skip_optional_whitespace(&mut tokens);
    skip_optional_comment(&mut tokens);
    if tokens.peek().is_none() {
        // There is no command on this line.
        return None;
    }
    let command = take_command(&mut tokens, line_number);
    // We could get away with not parsing the rest of the line, but it's good to
    // do, because there could be any kind of syntax errors lurking there...
    skip_optional_whitespace(&mut tokens);
    skip_optional_comment(&mut tokens);
    if let Some(_) = tokens.next() {
        panic!(
            "expected end of line. instead found another token. line: {}",
            line_number
        );
    }

    Some(command)
}

pub fn parse_lines<'a>(
    lines: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = Command> + 'a {
    lines
        .enumerate()
        .filter_map(|(line_idx, line)| parse_line(line, line_idx + 1))
        .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_c_command() {
        let mut tokens = tokenize_asm_line("M=M+1;JGT".to_string()).peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "M+1".to_string(),
                dest: Some("M".to_string()),
                jump: Some("JGT".to_string())
            }
        );

        let mut tokens = tokenize_asm_line("AMD=A|D;JLT".to_string()).peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "A|D".to_string(),
                dest: Some("AMD".to_string()),
                jump: Some("JLT".to_string())
            }
        );

        let mut tokens = tokenize_asm_line("M+1".to_string()).peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "M+1".to_string(),
                dest: None,
                jump: None
            }
        );

        let mut tokens = tokenize_asm_line("D&M;JGT".to_string()).peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "D&M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let mut tokens = tokenize_asm_line("!M;JGT".to_string()).peekable();
        let c_command = take_c_command(&mut tokens, 1);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "!M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let mut chars = tokenize_asm_line("MD=-A".to_string()).peekable();
        let c_command = take_c_command(&mut chars, 1);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "-A".to_string(),
                dest: Some("MD".to_string()),
                jump: None,
            }
        );
    }

    #[test]
    fn test_skip_optional_comment() {
        let mut tokens = tokenize_asm_line("// hey there".to_string()).peekable();
        skip_optional_comment(&mut tokens);
        let remaining = tokens.next();
        assert_eq!(remaining, None);

        let mut tokens = tokenize_asm_line("not a comment".to_string()).peekable();
        skip_optional_comment(&mut tokens);
        let result = tokens.next();
        assert_eq!(
            result,
            Some(Token {
                kind: TokenKind::Identifier("not".to_string()),
                length: 3
            })
        );
    }

    #[test]
    fn test_skip_optional_whitespace() {
        let mut tokens = tokenize_asm_line("      hello".to_string()).peekable();
        skip_optional_whitespace(&mut tokens);
        let remaining = tokens.next();
        assert_eq!(
            remaining,
            Some(Token {
                kind: TokenKind::Identifier("hello".to_string()),
                length: 5
            })
        );
    }

    #[test]
    fn test_skip_optional_whitespace_and_comment() {
        let mut tokens = tokenize_asm_line("      // this is a comment".to_string()).peekable();
        skip_optional_whitespace(&mut tokens);
        skip_optional_comment(&mut tokens);
        let remaining = tokens.next();
        assert_eq!(remaining, None);
    }

    #[test]
    fn test_take_a_command() {
        let mut tokens = tokenize_asm_line("@1234".to_string()).peekable();
        let a_command = take_a_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::ACommand(AValue::Numeric("1234".to_string()))
        );

        let mut tokens = tokenize_asm_line("@FOOBAR".to_string()).peekable();
        let a_command = take_a_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::ACommand(AValue::Symbolic("FOOBAR".to_string()))
        );
    }

    #[test]
    fn test_take_l_command() {
        let mut tokens = tokenize_asm_line("(TEST)".to_string()).peekable();
        let a_command = take_l_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::LCommand {
                identifier: "TEST".to_string()
            }
        );

        let mut tokens = tokenize_asm_line("(_TEST)".to_string()).peekable();
        let a_command = take_l_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::LCommand {
                identifier: "_TEST".to_string()
            }
        );

        let mut tokens = tokenize_asm_line("(T:E$S.T)".to_string()).peekable();
        let a_command = take_l_command(&mut tokens, 1);
        assert_eq!(
            a_command,
            Command::LCommand {
                identifier: "T:E$S.T".to_string()
            }
        );
    }

    #[test]
    fn test_parse() {
        let line = "";
        let result = parse_line(line, 1);
        assert_eq!(result, None);

        let line = "     ";
        let result = parse_line(line, 1);
        assert_eq!(result, None);

        let line = "  // hello this is a comment   ";
        let result = parse_line(line, 1);
        assert_eq!(result, None);

        let line = "// hello this is a comment";
        let result = parse_line(line, 1);
        assert_eq!(result, None);

        let line = "@1234";
        let result = parse_line(line, 1);
        assert_eq!(
            result,
            Some(Command::ACommand(AValue::Numeric("1234".to_string())))
        );

        let line = "   @1234  // here is a comment  ";
        let result = parse_line(line, 1);
        assert_eq!(
            result,
            Some(Command::ACommand(AValue::Numeric("1234".to_string())))
        );
    }

    #[test]
    #[should_panic(expected = "expected end of line. instead found another token")]
    fn test_parse_panic() {
        let line = "   @1234 blah blah blah";
        let result = parse_line(line, 1);
        assert_eq!(
            result,
            Some(Command::ACommand(AValue::Numeric("1234".to_string())))
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
        let result: Vec<Command> = parse_lines(source.lines()).collect();
        assert_eq!(
            result,
            vec![
                Command::ACommand(AValue::Numeric("1234".to_string())),
                Command::CCommand {
                    expr: "M+1".to_string(),
                    dest: Some("AMD".to_string()),
                    jump: Some("JGT".to_string())
                },
                Command::LCommand {
                    identifier: "FOOBAR".to_string()
                },
                Command::ACommand(AValue::Symbolic("FOOBAR".to_string())),
            ]
        );
    }
}
