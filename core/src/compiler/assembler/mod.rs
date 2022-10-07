mod codegen;
mod first_pass;
pub mod parser;
pub mod tokenizer;

use codegen::CodeGenerator;
use first_pass::first_pass;
use std::{fs, path::Path};

use self::parser::{parse, ASMInstruction};

pub fn assemble(source: Vec<ASMInstruction>, rom_depth: usize) -> Vec<String> {
    let first_pass_result = first_pass(source.into_iter());
    let mut code_generator = CodeGenerator::new(first_pass_result);
    let mut machine_instructions = code_generator.generate();
    let mut result = Vec::new();
    for _ in 0..rom_depth {
        if let Some(machine_code) = machine_instructions.next() {
            result.push(machine_code);
        } else {
            result.push("0000000000000000".to_string());
        }
    }
    result
}

pub fn assemble_file(source_path: &Path, dest_path: &Path, rom_depth: usize) {
    let string = fs::read_to_string(source_path).expect("failed to read source file");
    let parsed_instructions = parse(&string);
    let machine_code = assemble(parsed_instructions.collect(), rom_depth).join("\n");
    fs::write(dest_path, machine_code).expect("failed to write output");
}