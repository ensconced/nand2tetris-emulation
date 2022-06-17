use crate::compilers::jack_compiler::codegen::CodeGenerator;

use self::parser::parse;

use super::utils::source_modules::{get_source_modules, SourceModule};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod codegen;
mod parser;
mod tokenizer;

fn compile_source_module(source_module: &SourceModule) -> String {
    let class = parse(&source_module.source);
    let mut code_generator = CodeGenerator::new();
    code_generator.vm_code(class)
}

fn compile(src_path: &Path, dest_path: &Path) {
    let source_modules = get_source_modules(src_path).expect("failed to get source modules");
    for source_module in source_modules {
        let vm_code = compile_source_module(&source_module);
        let mut module_dest_path = if source_module.entrypoint_is_dir {
            dest_path.join(source_module.filename)
        } else {
            PathBuf::from(dest_path)
        };
        module_dest_path.set_extension("vm");
        fs::write(module_dest_path, vm_code).expect("failed to write compilation output")
    }
}
