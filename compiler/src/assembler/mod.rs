pub mod codegen;
mod first_pass;
pub mod parser;
pub mod tokenizer;

use codegen::CodeGenerator;
use first_pass::first_pass;
use std::{fs, path::Path};

use self::{
    codegen::AssemblyResult,
    parser::{parse, ASMInstruction},
};

pub fn assemble(source: &[ASMInstruction], rom_depth: usize) -> AssemblyResult {
    let first_pass_result = first_pass(source);
    let mut code_generator = CodeGenerator::new(first_pass_result);
    let mut assembly_result = code_generator.generate();
    while assembly_result.instructions.len() < rom_depth {
        assembly_result.instructions.push("0000000000000000".to_string());
    }
    assembly_result
}

pub fn assemble_file(source_path: &Path, dest_path: &Path, rom_depth: usize) {
    let string = fs::read_to_string(source_path).expect("failed to read source file");
    let parsed_instructions = parse(&string);
    let machine_code = assemble(&parsed_instructions, rom_depth).instructions.join("\n");
    fs::write(dest_path, machine_code).expect("failed to write output");
}
