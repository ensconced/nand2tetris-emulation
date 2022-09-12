use std::{collections::HashMap, path::PathBuf};

use self::{
    assembler::{assemble, parser::ASMInstruction},
    jack_compiler::{compile_jack, JackCompilerResult},
    utils::source_modules::SourceModule,
    vm_compiler::{
        codegen::{VMCompilerInput, VMCompilerResult},
        parser::Command,
    },
};

use crate::emulator::config;

pub mod assembler;
pub mod jack_compiler;
pub mod utils;
pub mod vm_compiler;

pub struct CompilerResult {
    pub jack_compiler_results: HashMap<PathBuf, JackCompilerResult>,
    pub vm_compiler_result: VMCompilerResult,
}

pub fn compile_to_machine_code(jack_code: Vec<SourceModule>) -> Vec<String> {
    let jack_compiler_results = compile_jack(jack_code);
    let vm_compiler_result = vm_compiler::codegen::generate_asm(
        jack_compiler_results
            .into_iter()
            .map(|jack_compiler_result| VMCompilerInput {
                commands: jack_compiler_result.commands,
                filename: jack_compiler_result.filename,
            })
            .collect(),
    );
    assemble(vm_compiler_result.instructions, config::ROM_DEPTH)
}
