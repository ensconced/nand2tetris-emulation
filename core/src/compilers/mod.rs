use std::ops::Deref;

use self::{
    assembler::assemble,
    jack_compiler::{codegen::generate_vm_code, parser::parse},
    utils::source_modules::get_source_modules,
    vm_compiler::ParsedModule,
};

use crate::emulator::config;

pub mod assembler;
pub mod jack_compiler;
pub mod utils;
pub mod vm_compiler;

use std::path::Path;

pub fn compile_to_machine_code(jack_code: Vec<&str>) -> String {
    let std_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../std_lib");

    let std_lib_source: Vec<_> = get_source_modules(&std_lib_dir)
        .expect("failed to get stdlib modules")
        .into_iter()
        .map(|stdlib_module| stdlib_module.source)
        .collect();

    let jack_classes: Vec<_> = std_lib_source
        .iter()
        .map(|source| source.deref())
        .chain(jack_code.into_iter())
        .map(|src| parse(src).class)
        .collect();

    let parsed_vm_modules: Vec<_> = jack_classes
        .into_iter()
        .map(generate_vm_code)
        .enumerate()
        .map(|(idx, commands)| ParsedModule {
            // TODO - use actual filenames here
            filename: format!("some_filename_{idx}").into(),
            commands: Box::new(commands.into_iter()),
        })
        .collect();

    let asm = vm_compiler::codegen::generate_asm(parsed_vm_modules);
    assemble(asm, config::ROM_DEPTH)
}
