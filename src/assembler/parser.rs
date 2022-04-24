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
non-digit-identifier-char = alphabetic OR : OR $ OR _
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

enum AValue<'a> {
    Numeric(i16),
    Symbolic(&'a str),
}

enum Command<'a> {
    ACommand(AValue<'a>),
    CCommand {
        expr: &'a str,
        dest: Option<&'a str>,
        jump: Option<&'a str>,
    },
    LCommand {
        identifier: &'a str,
    },
}

fn skip_whitespace(chars: &mut Peekable<impl Iterator<Item = char>>) {
    while let Some(next_ch) = chars.peek() {
        if next_ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

#[test]
fn test_skip_whitespace() {
    let str = "      hello";
    let mut chars = str.chars().peekable();
    skip_whitespace(&mut chars);
    let result: String = chars.collect();
    assert_eq!(result, "hello")
}

// fn parse(line: &str) -> Result<Option<Command>, ()> {
//     let mut chars = line.chars();
//     skip_whitespace(&mut chars);
//     Ok(None)
// }
