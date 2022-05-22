pub mod assembler;
mod parser_utils;
mod tokenizer;
pub mod vm_compiler;

use crate::config;

fn compile_jack_to_vm_code(jack_source: &str) -> String {
    todo!()
}

fn compile(jack_source: &str) -> String {
    let vm_code = compile_jack_to_vm_code(jack_source);
    let asm = vm_compiler::compile_to_asm(vm_code);
    assembler::assemble(asm, config::ROM_DEPTH)
}
