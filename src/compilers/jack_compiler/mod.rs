use crate::compilers::jack_compiler::codegen::CodeGenerator;

use self::parser::parse;

use super::utils::source_modules::{get_source_modules, SourceModule};
use std::path::Path;

mod codegen;
mod parser;
mod tokenizer;

fn compile_source_module(source_module: SourceModule) {
    let class = parse(&source_module.source);
    let code_generator = CodeGenerator::new(class);
    let vm_code = code_generator.vm_code();
    // TODO - write vm_code to corresponding path...
}

fn compile(src_path: &Path) {
    let source_modules = get_source_modules(src_path).expect("failed to get source modules");
    for source_module in source_modules {
        compile_source_module(source_module);
    }
}
