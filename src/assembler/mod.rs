mod codegen;
mod first_pass;
mod parser;
mod tokenizer;

use codegen::machine_codes;
use first_pass::first_pass;
use parser::parse_lines;
use std::{fs, path::Path};

fn assemble(source: String, rom_depth: usize) -> String {
    let first_pass_result = first_pass(parse_lines(source.lines()));
    let mut machine_codes = machine_codes(&first_pass_result);
    let mut result = String::new();
    for _ in 0..rom_depth {
        if let Some(machine_code) = machine_codes.next() {
            result.extend(machine_code.chars());
            result.push('\n');
        } else {
            result.extend("0000000000000000\n".chars());
        }
    }
    result
}

pub fn assemble_file(source_path: &Path, dest_path: &Path, rom_depth: usize) {
    let string = fs::read_to_string(source_path).expect("failed to read source file");
    let machine_code = assemble(string, rom_depth);
    fs::write(dest_path, machine_code).expect("failed to write output");
}
