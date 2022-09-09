use self::{
    assembler::assemble,
    jack_compiler::compile_jack,
    utils::source_modules::{get_source_modules, SourceModule},
    vm_compiler::CompiledJackFile,
};

use crate::emulator::config;

pub mod assembler;
pub mod jack_compiler;
pub mod utils;
pub mod vm_compiler;

use std::path::Path;

pub fn compile_to_machine_code(jack_code: Vec<&SourceModule>) -> Vec<String> {
    let std_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../std_lib");
    let std_lib_source: Vec<_> = get_source_modules(&std_lib_dir).expect("failed to get stdlib modules");
    let parsed_vm_modules: Vec<_> = std_lib_source
        .iter()
        .chain(jack_code.into_iter())
        .map(|source_module| {
            let jack_compiler_result = compile_jack(&source_module.filename, source_module.source.to_owned());
            CompiledJackFile {
                filename: &source_module.filename,
                commands: Box::new(jack_compiler_result.commands.into_iter()),
            }
        })
        .collect();

    let asm = vm_compiler::codegen::generate_asm(parsed_vm_modules);
    assemble(asm, config::ROM_DEPTH)
}
