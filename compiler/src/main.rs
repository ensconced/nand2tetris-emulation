mod asm_compressor;
mod assembler;
mod config;
mod fonts;
mod jack_compiler;
mod utils;
mod vm_compiler;

use assembler::codegen::AssemblyResult;
use clap::{Parser, Subcommand};
use config::ROM_DEPTH;
use jack_compiler::JackCompilerResult;
use serde::Serialize;
use std::{fs, path::Path};
use ts_rs::TS;
use utils::source_modules::SourceModule;
use vm_compiler::codegen::VMCompilerResult;
use {
    assembler::{assemble, assemble_file},
    jack_compiler::{compile_jack, jack_node_types::Class},
    vm_compiler::codegen::generate_asm,
};

#[derive(Default, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
struct CompilerResult {
    pub jack_compiler_result: JackCompilerResult,
    pub vm_compiler_result: VMCompilerResult,
    pub assembly_result: AssemblyResult,
}

// TODO - move into test module
pub fn compile_to_machine_code(jack_code: Vec<SourceModule>) -> Vec<String> {
    let jack_compiler_results = compile_jack(jack_code);
    let vm_compiler_result = vm_compiler::codegen::generate_asm(&jack_compiler_results.std_lib_commands, &jack_compiler_results.user_commands);
    assemble(&vm_compiler_result.instructions, config::ROM_DEPTH).instructions
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../web/bindings/")]
struct ParserVizData {
    source: String,
    parsed_class: Class,
}

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile jack code, generating JSON output including a sourcemap
    DebugCompile {
        dest_path: Option<String>,
        debug_output_path: Option<String>,
    },
    /// Compile assembly to machine code
    Assemble {
        source_path: Option<String>,
        dest_path: Option<String>,
    },
    /// Compile vm code to assembly
    Compile {
        source_path: Option<String>,
        dest_path: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::DebugCompile {
            dest_path: dest_path_maybe,
            debug_output_path: debug_output_path_maybe,
        } => {
            let debug_output_path = debug_output_path_maybe.as_ref().expect("debug output path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");

            let jack_compiler_result = compile_jack(vec![]);
            let vm_compiler_result = generate_asm(&jack_compiler_result.std_lib_commands, &jack_compiler_result.user_commands);
            let assembly_result = assemble(&vm_compiler_result.instructions, ROM_DEPTH);
            let compiler_result = CompilerResult {
                jack_compiler_result,
                vm_compiler_result,
                assembly_result,
            };
            let json = serde_json::to_string_pretty(&compiler_result).expect("failed to serialize jack compiler result");
            fs::write(debug_output_path, json).expect("failed to write result to debug output path");
            fs::write(dest_path, compiler_result.assembly_result.instructions.join("\n")).expect("failed to write result to dest path");
        }
        Commands::Assemble {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            println!("assembling {} to {}", source_path, dest_path);
            assemble_file(Path::new(source_path), Path::new(dest_path), config::ROM_DEPTH);
        }
        Commands::Compile {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            println!("assembling {} to {}", source_path, dest_path);
            vm_compiler::compile_files(Path::new(source_path), Path::new(dest_path)).unwrap();
        }
    }
}
