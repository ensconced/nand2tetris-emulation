use super::first_pass::FirstPassResult;
use super::parser::{AValue, Command};
use std::collections::HashMap;

fn expression_code(expr: &String) -> &'static str {
    match expr.as_str() {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "1+D" => "0011111",
        "A+1" => "0110111",
        "1+A" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "A+D" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "A&D" => "0000000",
        "D|A" => "0010101",
        "A|D" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "1+M" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "M+D" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "M&D" => "1000000",
        "D|M" => "1010101",
        "M|D" => "1010101",
        _ => panic!("unrecognized expression"),
    }
}

fn dest_code(dest_opt: Option<&String>) -> &'static str {
    match dest_opt {
        None => "000",
        Some(string) => {
            let str = string.as_str();
            match str {
                "A" => "100",
                "D" => "010",
                "M" => "001",
                "AD" => "110",
                "AM" => "101",
                "DA" => "110",
                "DM" => "011",
                "MA" => "101",
                "MD" => "011",
                "AMD" => "111",
                "ADM" => "111",
                "DAM" => "111",
                "DMA" => "111",
                "MAD" => "111",
                "MDA" => "111",
                _ => panic!("unrecognized destination"),
            }
        }
    }
}

fn jump_code(jump_opt: Option<&String>) -> &'static str {
    match jump_opt {
        None => "000",
        Some(string) => match string.as_str() {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => panic!("unrecognized jump"),
        },
    }
}

fn c_command_code(expr: &String, dest: Option<&String>, jump: Option<&String>) -> String {
    format!(
        "111{}{}{}",
        expression_code(expr),
        dest_code(dest),
        jump_code(jump)
    )
}

fn numeric_a_command_code(num_string: &String) -> String {
    let num = i16::from_str_radix(num_string, 10).expect("failed to parse numeric a-command");
    if num < 0 {
        // The most significant bit (msb) is reserved for distinguishing between
        // A-commands and C-commands. This means the msb is always 0 for
        // A-commands and therefore you can't use negative numbers in
        // A-commands.
        panic!("negative numbers are not allowed in a-commands");
    }
    format!("{:016b}", num)
}

fn symbolic_a_command_code(sym: &String, resolved_symbols: &HashMap<String, usize>) -> String {
    let num = resolved_symbols
        .get(sym)
        .expect("symbol not present in resolved_symbols");
    if let Ok(num_16) = i16::try_from(*num) {
        format!("{:016b}", num_16)
    } else {
        panic!("failed to resolve symbolic a-command to valid index");
    }
}

fn machine_code(command: &Command, resolved_symbols: &HashMap<String, usize>) -> String {
    match command {
        Command::CCommand { expr, dest, jump } => {
            c_command_code(expr, dest.as_ref(), jump.as_ref())
        }
        Command::ACommand(AValue::Numeric(num)) => numeric_a_command_code(num),
        Command::ACommand(AValue::Symbolic(sym)) => symbolic_a_command_code(sym, resolved_symbols),
        _ => panic!("unexpected l_command remaining after first pass"),
    }
}

pub fn machine_codes<'a>(
    first_pass_result: &'a FirstPassResult,
) -> impl Iterator<Item = String> + 'a {
    first_pass_result
        .commands_without_labels
        .iter()
        .map(|command| machine_code(command, &first_pass_result.resolved_symbols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_command_code() {
        assert_eq!(
            c_command_code(
                &"M+1".to_string(),
                Some(&"A".to_string()),
                Some(&"JGT".to_string()),
            ),
            "1111110111100001".to_string()
        );
    }
    #[test]
    fn test_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code(&"1".to_string()), "0000000000000001");
        assert_eq!(
            numeric_a_command_code(&"1234".to_string()),
            "0000010011010010"
        );
    }

    #[test]
    #[should_panic(expected = "negative numbers are not allowed in a-commands")]
    fn test_negative_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code(&"-1234".to_string()), "whatever");
    }

    #[test]
    #[should_panic]
    fn test_too_big_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code(&"100000".to_string()), "whatever");
    }

    #[test]
    fn test_symbolic_a_command_code() {
        let resolved_symbols = HashMap::from([("foo".to_string(), 32)]);
        let code = symbolic_a_command_code(&"foo".to_string(), &resolved_symbols);
        assert_eq!(code, "0000000000100000");
    }

    #[test]
    #[should_panic(expected = "failed to resolve symbolic a-command to valid index")]
    fn test_too_big_symbolic_a_command_code() {
        let resolved_symbols = HashMap::from([("foo".to_string(), 1000000)]);
        let code = symbolic_a_command_code(&"foo".to_string(), &resolved_symbols);
        assert_eq!(code, "whatever");
    }
}