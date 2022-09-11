use self::{assembler::assemble, jack_compiler::compile_jack, utils::source_modules::SourceModule};

use crate::emulator::config;

pub mod assembler;
pub mod jack_compiler;
pub mod utils;
pub mod vm_compiler;

pub fn compile_to_machine_code(jack_code: Vec<SourceModule>) -> Vec<String> {
    let jack_compiler_results = compile_jack(jack_code);
    let asm = vm_compiler::codegen::generate_asm(jack_compiler_results);
    assemble(asm.join("\n"), config::ROM_DEPTH)
}
