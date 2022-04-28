use std::iter::Peekable;

#[derive(PartialEq, Debug)]
enum AValue {
    Numeric(String),
    Symbolic(String),
}

#[derive(PartialEq, Debug)]
enum Command {
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

fn skip_optional_comment(chars: &mut Peekable<impl Iterator<Item = char>>) {
    let mut slash_count = 0;
    while let Some(next_ch) = chars.peek() {
        if *next_ch == '/' || slash_count >= 2 {
            slash_count = slash_count + 1;
            chars.next();
        } else if slash_count > 0 {
            panic!("failed to parse comment");
        } else {
            break;
        }
    }
}

fn skip_optional_whitespace(chars: &mut Peekable<impl Iterator<Item = char>>) {
    while let Some(next_ch) = chars.peek() {
        if next_ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn is_valid_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ":$_.".contains(ch)
}

fn is_valid_first_place_identifier_char(ch: char) -> bool {
    is_valid_identifier_char(ch) && !ch.is_ascii_digit()
}

fn take_identifier(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut result = String::new();
    if let Some(first_char) = chars.next() {
        dbg!(first_char);
        if is_valid_first_place_identifier_char(first_char) {
            result.push(first_char);
        } else {
            panic!("failed to parse identifier");
        }
    } else {
        panic!("failed to parse identifier - unexpected end of input");
    }

    while let Some(ch) = chars.peek() {
        if is_valid_identifier_char(*ch) {
            result.push(chars.next().unwrap())
        } else {
            break;
        }
    }
    result
}

fn take_number(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut result = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            result.push(chars.next().unwrap())
        } else {
            break;
        }
    }
    result
}

fn take_a_value(chars: &mut Peekable<impl Iterator<Item = char>>) -> AValue {
    if let Some(next_ch) = chars.peek() {
        if next_ch.is_ascii_digit() {
            AValue::Numeric(take_number(chars))
        } else {
            AValue::Symbolic(take_identifier(chars))
        }
    } else {
        panic!("failed to parse a_value");
    }
}

fn take_a_command(chars: &mut Peekable<impl Iterator<Item = char>>) -> Command {
    chars.next(); // @
    let a_value = take_a_value(chars);
    Command::ACommand(a_value)
}

fn take_l_command(chars: &mut Peekable<impl Iterator<Item = char>>) -> Command {
    chars.next();
    let identifier = take_identifier(chars);
    if let Some(ch) = chars.next() {
        if ch == ')' {
            Command::LCommand { identifier }
        } else {
            panic!("failed to parse l command");
        }
    } else {
        panic!("failed to parse l command");
    }
}

fn take_remainder_of_destination(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut result = String::new();
    while let Some(ch) = chars.peek() {
        if *ch == '=' {
            chars.next();
            break;
        } else if "AMD".contains(*ch) {
            let keep = chars.next().unwrap();
            result.push(keep);
        } else {
            break;
        }
    }
    result
}

fn take_expression(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    first_ch_maybe: Option<char>,
) -> String {
    let first_ch = first_ch_maybe.unwrap_or_else(|| {
        chars
            .next()
            .expect("failed to parse expression - unexpected end of input")
    });

    if "01ADM".contains(first_ch) {
        let mut result = String::new();
        result.push(first_ch);
        // Could be either a simple value or a binary expression.
        if let Some(second_ch) = chars.peek() {
            if "-+|&".contains(*second_ch) {
                // Must be binary expression. TODO - clean this up by using
                // a separate take_remainder_of_binary_expression fn? Or at
                // least take_operator etc...?
                result.push(chars.next().unwrap());
                result.push(chars.next().unwrap());
            }
        }
        result
    } else if "-!".contains(first_ch) {
        format!("{}{}", first_ch, take_identifier(chars))
    } else {
        panic!(
            "failed to parse expression - invalid first character {:?}",
            first_ch
        );
    }
}

fn take_optional_jump(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    let valid_jumps: Vec<String> = vec!["JGT", "JEQ", "JGE", "JLT", "JNE", "JLE", "JMP"]
        .into_iter()
        .map(String::from)
        .collect();
    if let Some(first_ch) = chars.peek() {
        if *first_ch == ';' {
            chars.next(); // ;
            skip_optional_whitespace(chars);
            let jump: String = chars.take(3).collect();
            if valid_jumps.contains(&jump) {
                Some(jump)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn take_c_command(chars: &mut Peekable<impl Iterator<Item = char>>) -> Command {
    if let Some(first_ch) = chars.next() {
        let mut has_destination = false;
        let mut dest = None;
        if "AMD".contains(first_ch) {
            // This is a bit tricky - this could either be the destination or
            // the expression. Crucially, we don't allow whitespace separating
            // the destination characters, or between the destination characters
            // and the equals sign.
            if let Some(second_ch) = chars.peek() {
                if "AMD=".contains(*second_ch) {
                    has_destination = true;
                }
            }
        }

        if has_destination {
            dest = Some(format!(
                "{}{}",
                first_ch,
                take_remainder_of_destination(chars)
            ));
        }

        let expr = take_expression(
            chars,
            if has_destination {
                None
            } else {
                Some(first_ch)
            },
        );

        skip_optional_whitespace(chars);

        let jump = take_optional_jump(chars);

        Command::CCommand { expr, dest, jump }
    } else {
        panic!("failed to parse c_command");
    }
}

fn take_command(chars: &mut Peekable<impl Iterator<Item = char>>) -> Command {
    match chars.peek() {
        Some('@') => take_a_command(chars),
        Some('(') => take_l_command(chars),
        Some(_) => take_c_command(chars),
        None => panic!("failed to parse command"),
    }
}

fn parse(line: &str) -> Result<Option<Command>, ()> {
    let mut chars = line.chars().peekable();
    skip_optional_whitespace(&mut chars);
    skip_optional_comment(&mut chars);
    if chars.peek().is_none() {
        // There is no command on this line.
        return Ok(None);
    }
    let command = take_command(&mut chars);

    // We could get away with not parsing the rest of the line, but it's good to
    // do, because there could be any kind of syntax errors lurking there...
    skip_optional_whitespace(&mut chars);
    skip_optional_comment(&mut chars);
    if let Some(remaining_char) = chars.next() {
        panic!(
            "unexpected character \"{}\" instead of end of line",
            remaining_char
        );
    }

    Ok(Some(command))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_c_command() {
        let str = "M=M+1;JGT";
        let mut chars = str.chars().peekable();
        let c_command = take_c_command(&mut chars);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "M+1".to_string(),
                dest: Some("M".to_string()),
                jump: Some("JGT".to_string())
            }
        );

        let str = "AMD=A|D;JLT";
        let mut chars = str.chars().peekable();
        let c_command = take_c_command(&mut chars);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "A|D".to_string(),
                dest: Some("AMD".to_string()),
                jump: Some("JLT".to_string())
            }
        );

        let str = "M+1";
        let mut chars = str.chars().peekable();
        let c_command = take_c_command(&mut chars);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "M+1".to_string(),
                dest: None,
                jump: None
            }
        );

        let str = "D&M;JGT";
        let mut chars = str.chars().peekable();
        let c_command = take_c_command(&mut chars);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "D&M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let str = "!M;JGT";
        let mut chars = str.chars().peekable();
        let c_command = take_c_command(&mut chars);
        assert_eq!(
            c_command,
            Command::CCommand {
                expr: "!M".to_string(),
                dest: None,
                jump: Some("JGT".to_string()),
            }
        );

        let str = "MD=-A";
        let mut chars = str.chars().peekable();
        let c_command = take_c_command(&mut chars);
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
        let str = "// hey there";
        let mut chars = str.chars().peekable();
        skip_optional_comment(&mut chars);
        let result: String = chars.collect();
        assert_eq!(result, "");

        let str = "not a comment";
        let mut chars = str.chars().peekable();
        skip_optional_comment(&mut chars);
        let result: String = chars.collect();
        assert_eq!(result, "not a comment");
    }

    #[test]
    fn test_skip_optional_whitespace() {
        let str = "      hello";
        let mut chars = str.chars().peekable();
        skip_optional_whitespace(&mut chars);
        let result: String = chars.collect();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_skip_optional_whitespace_and_comment() {
        let str = "      // this is a comment";
        let mut chars = str.chars().peekable();
        skip_optional_whitespace(&mut chars);
        skip_optional_comment(&mut chars);
        let result: String = chars.collect();
        assert_eq!(result, "");
    }

    #[test]
    fn test_take_a_command() {
        let str = "@1234";
        let mut chars = str.chars().peekable();
        let a_command = take_a_command(&mut chars);
        assert_eq!(
            a_command,
            Command::ACommand(AValue::Numeric("1234".to_string()))
        );
    }

    #[test]
    fn test_take_l_command() {
        let str = "(TEST)";
        let mut chars = str.chars().peekable();
        let a_command = take_l_command(&mut chars);
        assert_eq!(
            a_command,
            Command::LCommand {
                identifier: "TEST".to_string()
            }
        );
    }

    #[test]
    fn test_parse() {
        let line = "";
        let result = parse(line);
        assert_eq!(result, Ok(None));

        let line = "     ";
        let result = parse(line);
        assert_eq!(result, Ok(None));

        let line = "  // hello this is a comment   ";
        let result = parse(line);
        assert_eq!(result, Ok(None));

        let line = "// hello this is a comment";
        let result = parse(line);
        assert_eq!(result, Ok(None));

        let line = "@1234";
        let result = parse(line);
        assert_eq!(
            result,
            Ok(Some(Command::ACommand(AValue::Numeric("1234".to_string()))))
        );

        let line = "   @1234  // here is a comment  ";
        let result = parse(line);
        assert_eq!(
            result,
            Ok(Some(Command::ACommand(AValue::Numeric("1234".to_string()))))
        );
    }

    #[test]
    #[should_panic(expected = "unexpected character \"b\" instead of end of line")]
    fn test_parse_panic() {
        let line = "   @1234 blah blah blah";
        let result = parse(line);
        assert_eq!(
            result,
            Ok(Some(Command::ACommand(AValue::Numeric("1234".to_string()))))
        );
    }
}
