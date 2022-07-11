use super::first_pass::FirstPassResult;
use super::parser::{
    AValue::*,
    Command::{self, *},
};
use std::collections::HashMap;

fn predefined_symbol_code(sym: &str) -> Option<usize> {
    match sym {
        "SP" => Some(0),
        "LCL" => Some(1),
        "ARG" => Some(2),
        "THIS" => Some(3),
        "THAT" => Some(4),
        "R0" => Some(0),
        "R1" => Some(1),
        "R2" => Some(2),
        "R3" => Some(3),
        "R4" => Some(4),
        "R5" => Some(5),
        "R6" => Some(6),
        "R7" => Some(7),
        "R8" => Some(8),
        "R9" => Some(9),
        "R10" => Some(10),
        "R11" => Some(11),
        "R12" => Some(12),
        "R13" => Some(13),
        "R14" => Some(14),
        "R15" => Some(15),
        "SCREEN" => Some(18432),
        "KBD" => Some(26624),
        _ => None,
    }
}

fn expression_code(expr: &str) -> &'static str {
    match expr {
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
        "A+1" | "1+A" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" | "A+D" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" | "A&D" => "0000000",
        "D|A" | "A|D" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" | "1+M" => "1110111",
        "M-1" => "1110010",
        "D+M" | "M+D" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" | "M&D" => "1000000",
        "D|M" | "M|D" => "1010101",
        _ => panic!("unrecognized expression {}", expr),
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
            _ => panic!("unrecognized jump \"{}\"", string),
        },
    }
}

fn c_command_code(expr: &str, dest: Option<&String>, jump: Option<&String>) -> String {
    format!(
        "111{}{}{}",
        expression_code(expr),
        dest_code(dest),
        jump_code(jump)
    )
}

fn numeric_a_command_code(num_string: &str) -> String {
    let num = num_string
        .parse::<i16>()
        .expect("failed to parse numeric a-command");
    if num < 0 {
        // The most significant bit (msb) is reserved for distinguishing between
        // A-commands and C-commands. This means the msb is always 0 for
        // A-commands and therefore you can't use negative numbers in
        // A-commands.
        panic!("negative numbers are not allowed in a-commands");
    }
    format!("{:016b}", num)
}

pub struct CodeGenerator {
    resolved_symbols: HashMap<String, usize>,
    commands_without_labels: Vec<Command>,
    address_next_static_variable: usize,
}

impl CodeGenerator {
    pub fn new(first_pass_result: FirstPassResult) -> Self {
        Self {
            address_next_static_variable: 16,
            resolved_symbols: first_pass_result.resolved_symbols,
            commands_without_labels: first_pass_result.commands_without_labels,
        }
    }

    pub fn generate(&mut self) -> impl Iterator<Item = String> + '_ {
        self.commands_without_labels
            .iter()
            .map(|command| match command {
                C { expr, dest, jump } => c_command_code(expr, dest.as_ref(), jump.as_ref()),
                A(Numeric(num)) => numeric_a_command_code(num),
                A(Symbolic(sym)) => {
                    let index = predefined_symbol_code(sym)
                        .or_else(|| self.resolved_symbols.get(sym).copied())
                        .unwrap_or_else(|| {
                            let address = self.address_next_static_variable;
                            if address > 255 {
                                panic!("too many static variables - ran out of place while trying to place \"{}\"", sym)
                            }
                            self.resolved_symbols.insert(sym.to_string(), address);
                            self.address_next_static_variable += 1;
                            address
                        });
                    if let Ok(num_16) = i16::try_from(index) {
                        format!("{:016b}", num_16)
                    } else {
                        panic!("failed to resolve symbolic a-command to valid index");
                    }
                }
                L { identifier: _ } => {
                    panic!("unexpected l_command remaining after first pass")
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_command_code() {
        assert_eq!(
            c_command_code("M+1", Some(&"A".to_string()), Some(&"JGT".to_string()),),
            "1111110111100001".to_string()
        );
    }
    #[test]
    fn test_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code("1"), "0000000000000001");
        assert_eq!(numeric_a_command_code("1234"), "0000010011010010");
    }

    #[test]
    #[should_panic(expected = "negative numbers are not allowed in a-commands")]
    fn test_negative_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code("-1234"), "whatever");
    }

    #[test]
    #[should_panic]
    fn test_too_big_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code("100000"), "whatever");
    }

    #[test]
    fn test_label_symbol_a_command_code() {
        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("foo".to_string(), 32)]),
            commands_without_labels: vec![A(Symbolic("foo".to_string()))],
        };
        let mut code_generator = CodeGenerator::new(first_pass_result);
        let instruction = code_generator.generate().next().unwrap();
        assert_eq!(instruction, "0000000000100000");
    }

    #[test]
    fn test_variable_symbol_a_command_code() {
        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("i_am_a_label_symbol".to_string(), 255)]),
            commands_without_labels: vec![
                A(Symbolic("foo".to_string())),
                A(Symbolic("bar".to_string())),
                A(Symbolic("i_am_a_label_symbol".to_string())),
                A(Symbolic("baz".to_string())),
                A(Symbolic("foo".to_string())),
                A(Symbolic("bar".to_string())),
            ],
        };
        let mut code_generator = CodeGenerator::new(first_pass_result);
        let instructions: Vec<String> = code_generator.generate().collect();
        assert_eq!(
            instructions,
            vec![
                "0000000000010000",
                "0000000000010001",
                "0000000011111111",
                "0000000000010010",
                "0000000000010000",
                "0000000000010001",
            ]
        );
    }

    #[test]
    fn test_predefined_variable_symbol_a_command_code() {
        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("foo".to_string(), 32)]),
            commands_without_labels: vec![A(Symbolic("SCREEN".to_string()))],
        };
        let mut code_generator = CodeGenerator::new(first_pass_result);
        let instruction = code_generator.generate().next().unwrap();
        assert_eq!(instruction, "0100000000000000");
    }

    #[test]
    #[should_panic(expected = "failed to resolve symbolic a-command to valid index")]
    fn test_too_big_symbolic_a_command_code() {
        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("foo".to_string(), 1000000)]),
            commands_without_labels: vec![A(Symbolic("foo".to_string()))],
        };
        let mut code_generator = CodeGenerator::new(first_pass_result);
        code_generator.generate().count(); // .count() is just to consume the iterator
    }
}
