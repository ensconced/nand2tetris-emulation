/*

line =  {whitespace}, [command], {whitespace}, [comment]
whitespace = space OR tab
command = a_command OR c_command OR l_command
a_command = @, a_value
a_value = number OR identifier
number = non-zero-digit, {digit}
non-zero-digit = 1 OR 2 OR 3 OR 4 OR 5 OR 6 OR 7 OR 8 OR 9
digit = 0 OR non-zero-digit
identifier = non-digit-identifier-char, {identifier-char}
non-digit-identifier-char = alphabetic OR : OR $ OR _ OR .
identifier-char = digit OR non-digit-identifier-char
c_command = [dest], {whitespace}, expr, {whitespace}, [jump]
dest = location, {whitespace}, =, {whitespace}
location = single_location, [single_location], [single_location]
single_location = A OR D OR M
simple_val = digit OR single_location
expr = simple_val OR unary_expr OR binary_expr
unary_expr = unary_op, simple_val
unary_op = - OR !
binary_expr = simple_val OR binary_op OR simple_val
binary_op = + OR & OR - OR |
jump = JGT OR JEQ OR JGE OR JLT OR JNE OR JLE OR JMP
l_command = (, identifier, )

*/

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

fn skip_optional_whitespace(chars: &mut Peekable<impl Iterator<Item = char>>) {
    while let Some(next_ch) = chars.peek() {
        if next_ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
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

fn is_valid_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ":$_.".contains(ch)
}

fn is_valid_first_place_identifier_char(ch: char) -> bool {
    is_valid_identifier_char(ch) && !ch.is_ascii_digit()
}

fn take_identifier(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut result = String::new();
    if let Some(first_char) = chars.next() {
        if is_valid_first_place_identifier_char(first_char) {
            result.push(first_char);
        } else {
            panic!("failed to parse identifier");
        }
    } else {
        panic!("failed to parse identifier");
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

fn take_l_command(chars: &mut Peekable<impl Iterator<Item = char>>) -> Command {
    chars.next(); // (
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

fn take_c_command(chars: &mut Peekable<impl Iterator<Item = char>>) -> Command {
    todo!()
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
        // there is no command on this line
        return Ok(None);
    }
    let command = take_command(&mut chars);

    // we could get away with not parsing the rest of the line, but it's good to
    // do, because there could be any kind of syntax errors there...
    skip_optional_whitespace(&mut chars);
    skip_optional_comment(&mut chars);

    Ok(Some(command))
}
