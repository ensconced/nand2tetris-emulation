use serde::Serialize;
use ts_rs::TS;

use self::{
    assembler::assemble,
    jack_compiler::{compile_jack, JackCompilerResult},
    utils::source_modules::SourceModule,
    vm_compiler::codegen::VMCompilerResult,
};

use crate::emulator::config;

pub mod assembler;
pub mod jack_compiler;
pub mod utils;
pub mod vm_compiler;

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct CompilerResult {
    pub jack_compiler_result: JackCompilerResult,
    pub vm_compiler_result: VMCompilerResult,
}

// TODO - move into test module
pub fn compile_to_machine_code(jack_code: Vec<SourceModule>) -> Vec<String> {
    let jack_compiler_results = compile_jack(jack_code);
    let vm_compiler_result = vm_compiler::codegen::generate_asm(&jack_compiler_results.vm_commands);
    assemble(vm_compiler_result.instructions, config::ROM_DEPTH)
}
