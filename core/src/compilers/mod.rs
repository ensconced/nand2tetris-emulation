use self::{
    assembler::assemble,
    jack_compiler::{codegen::generate_vm_code, parser::parse, sourcemap::SourceMap, tokenizer::token_defs},
    utils::{
        source_modules::{get_source_modules, SourceModule},
        tokenizer::Tokenizer,
    },
    vm_compiler::ParsedModule,
};

use crate::emulator::config;

pub mod assembler;
pub mod jack_compiler;
pub mod utils;
pub mod vm_compiler;

use std::path::Path;

pub fn compile_to_machine_code(jack_code: Vec<&SourceModule>) -> String {
    let std_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../std_lib");

    let std_lib_source: Vec<_> = get_source_modules(&std_lib_dir).expect("failed to get stdlib modules");

    let parsed_vm_modules: Vec<_> = std_lib_source
        .iter()
        .chain(jack_code.into_iter())
        .map(|source_module| {
            let mut sourcemap = SourceMap::new();
            let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(&source_module.source);
            // TODO - should we just use a Path for SourceModule from the start to avoid having to do this conversion?
            let class = parse(Path::new(&source_module.filename), &tokens, &mut sourcemap);
            generate_vm_code(class, &mut sourcemap)
        })
        .enumerate()
        .map(|(idx, commands)| {
            ParsedModule {
                // TODO - use actual filenames here
                filename: format!("some_filename_{idx}").into(),
                commands: Box::new(commands.into_iter()),
            }
        })
        .collect();

    let asm = vm_compiler::codegen::generate_asm(parsed_vm_modules);
    assemble(asm, config::ROM_DEPTH)
}
