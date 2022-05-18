mod codegen;
mod first_pass;
pub mod parser;
pub mod tokenizer;

use codegen::CodeGenerator;
use first_pass::first_pass;
use parser::parse_lines;
use std::{fs, path::Path};

pub fn assemble(source: String, rom_depth: usize) -> String {
    let first_pass_result = first_pass(parse_lines(&source));
    let mut code_generator = CodeGenerator::new(first_pass_result);
    let mut machine_codes = code_generator.generate();
    let mut result = String::new();
    for _ in 0..rom_depth {
        if let Some(machine_code) = machine_codes.next() {
            result.push_str(&machine_code);
            result.push('\n');
        } else {
            result.push_str("0000000000000000\n");
        }
    }
    result
}

pub fn assemble_file(source_path: &Path, dest_path: &Path, rom_depth: usize) {
    let string = fs::read_to_string(source_path).expect("failed to read source file");
    let machine_code = assemble(string, rom_depth);
    fs::write(dest_path, machine_code).expect("failed to write output");
}
