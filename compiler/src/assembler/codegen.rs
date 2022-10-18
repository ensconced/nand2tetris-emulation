use serde::Serialize;
use ts_rs::TS;

use super::first_pass::FirstPassResult;
use super::parser::{
    ASMInstruction::{self, *},
    AValue::*,
};
use std::collections::HashMap;

fn predefined_symbol_code(sym: &str) -> Option<u16> {
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
        "GLYPHS" => Some(26625),
        _ => None,
    }
}

fn expression_code(expr: &str) -> u16 {
    match expr {
        "0" => 0b0101010,
        "1" => 0b0111111,
        "-1" => 0b0111010,
        "D" => 0b0001100,
        "A" => 0b0110000,
        "!D" => 0b0001101,
        "!A" => 0b0110001,
        "-D" => 0b0001111,
        "-A" => 0b0110011,
        "D+1" => 0b0011111,
        "1+D" => 0b0011111,
        "A+1" | "1+A" => 0b0110111,
        "D-1" => 0b0001110,
        "A-1" => 0b0110010,
        "D+A" | "A+D" => 0b0000010,
        "D-A" => 0b0010011,
        "A-D" => 0b0000111,
        "D&A" | "A&D" => 0b0000000,
        "D|A" | "A|D" => 0b0010101,
        "M" => 0b1110000,
        "!M" => 0b1110001,
        "-M" => 0b1110011,
        "M+1" | "1+M" => 0b1110111,
        "M-1" => 0b1110010,
        "D+M" | "M+D" => 0b1000010,
        "D-M" => 0b1010011,
        "M-D" => 0b1000111,
        "D&M" | "M&D" => 0b1000000,
        "D|M" | "M|D" => 0b1010101,
        _ => panic!("unrecognized expression {}", expr),
    }
}

fn dest_code(dest_opt: Option<&String>) -> u16 {
    match dest_opt {
        None => 0b000,
        Some(string) => {
            let str = string.as_str();
            match str {
                "A" => 0b100,
                "D" => 0b010,
                "M" => 0b001,
                "AD" => 0b110,
                "AM" => 0b101,
                "DA" => 0b110,
                "DM" => 0b011,
                "MA" => 0b101,
                "MD" => 0b011,
                "AMD" => 0b111,
                "ADM" => 0b111,
                "DAM" => 0b111,
                "DMA" => 0b111,
                "MAD" => 0b111,
                "MDA" => 0b111,
                _ => panic!("unrecognized destination"),
            }
        }
    }
}

fn jump_code(jump_opt: Option<&String>) -> u16 {
    match jump_opt {
        None => 0b000,
        Some(string) => match string.as_str() {
            "JGT" => 0b001,
            "JEQ" => 0b010,
            "JGE" => 0b011,
            "JLT" => 0b100,
            "JNE" => 0b101,
            "JLE" => 0b110,
            "JMP" => 0b111,
            _ => panic!("unrecognized jump \"{}\"", string),
        },
    }
}

fn c_command_code(expr: &str, dest: Option<&String>, jump: Option<&String>) -> u16 {
    (111 << 13) | (expression_code(expr) << 6) | (dest_code(dest) << 3) | jump_code(jump)
}

fn numeric_a_command_code(num_string: &str) -> u16 {
    let num = num_string.parse::<i16>().expect("failed to parse numeric a-command");
    if num < 0 {
        // The most significant bit (msb) is reserved for distinguishing between
        // A-commands and C-commands. This means the msb is always 0 for
        // A-commands and therefore you can't use negative numbers in
        // A-commands.
        panic!("negative numbers are not allowed in a-commands");
    }
    num as u16
}

pub struct CodeGenerator<'a> {
    resolved_symbols: HashMap<&'a str, u16>,
    commands: &'a [ASMInstruction],
    address_next_static_variable: u16,
}

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
pub struct AssemblyResult {
    pub instructions: Vec<u16>,
    pub sourcemap: Vec<usize>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(first_pass_result: FirstPassResult<'a>, commands: &'a [ASMInstruction]) -> Self {
        Self {
            address_next_static_variable: 16,
            resolved_symbols: first_pass_result.resolved_symbols,
            commands,
        }
    }

    fn machine_code(&mut self, command: &'a ASMInstruction) -> Option<u16> {
        match command {
            C { expr, dest, jump } => Some(c_command_code(expr, dest.as_ref(), jump.as_ref())),
            A(Numeric(num)) => Some(numeric_a_command_code(num)),
            A(Symbolic(sym)) => {
                let index = predefined_symbol_code(sym)
                    .or_else(|| self.resolved_symbols.get(sym.as_str()).copied())
                    .unwrap_or_else(|| {
                        let address = self.address_next_static_variable;
                        if address > 255 {
                            panic!("too many static variables - ran out of place while trying to place \"{}\"", sym)
                        }
                        self.resolved_symbols.insert(sym.as_str(), address);
                        self.address_next_static_variable += 1;
                        address
                    });
                Some(index)
            }
            L { identifier: _ } => None,
        }
    }

    pub fn generate(&mut self) -> AssemblyResult {
        let mut instructions = vec![];
        let mut sourcemap = vec![];

        for (command_idx, command) in self.commands.iter().enumerate() {
            if let Some(instruction) = self.machine_code(command) {
                sourcemap.push(command_idx);
                instructions.push(instruction);
            }
        }
        AssemblyResult { instructions, sourcemap }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_command_code() {
        assert_eq!(
            c_command_code("M+1", Some(&"A".to_string()), Some(&"JGT".to_string()),),
            0b1111110111100001
        );
    }
    #[test]
    fn test_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code("1"), 0b0000000000000001);
        assert_eq!(numeric_a_command_code("1234"), 0b0000010011010010);
    }

    #[test]
    #[should_panic(expected = "negative numbers are not allowed in a-commands")]
    fn test_negative_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code("-1234"), 1);
    }

    #[test]
    #[should_panic]
    fn test_too_big_numeric_a_command_code() {
        assert_eq!(numeric_a_command_code("100000"), 1);
    }

    #[test]
    fn test_label_symbol_a_command_code() {
        let commands = vec![A(Symbolic("foo".to_string()))];
        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("foo", 32)]),
        };
        let mut code_generator = CodeGenerator::new(first_pass_result, &commands);
        let AssemblyResult { instructions, .. } = code_generator.generate();
        assert_eq!(instructions[0], 0b0000000000100000);
    }

    #[test]
    fn test_variable_symbol_a_command_code() {
        let commands: Vec<_> = vec![
            A(Symbolic("foo".to_string())),
            A(Symbolic("bar".to_string())),
            A(Symbolic("i_am_a_label_symbol".to_string())),
            A(Symbolic("baz".to_string())),
            A(Symbolic("foo".to_string())),
            A(Symbolic("bar".to_string())),
        ];

        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("i_am_a_label_symbol", 255)]),
        };
        let mut code_generator = CodeGenerator::new(first_pass_result, &commands);
        assert_eq!(
            code_generator.generate().instructions,
            vec![
                0b0000000000010000,
                0b0000000000010001,
                0b0000000011111111,
                0b0000000000010010,
                0b0000000000010000,
                0b0000000000010001,
            ]
        );
    }

    #[test]
    fn test_predefined_variable_symbol_a_command_code() {
        let commands = vec![A(Symbolic("SCREEN".to_string()))];
        let first_pass_result = FirstPassResult {
            resolved_symbols: HashMap::from([("foo", 32)]),
        };
        let mut code_generator = CodeGenerator::new(first_pass_result, &commands);
        assert_eq!(code_generator.generate().instructions[0], 0b0100100000000000);
    }
}
