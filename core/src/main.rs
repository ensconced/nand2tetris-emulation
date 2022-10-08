mod compiler;
mod emulator;
mod fonts;

use clap::{Parser, Subcommand};
use compiler::{
    assembler::{assemble, assemble_file},
    jack_compiler::{compile_jack, jack_node_types::Class},
    vm_compiler::{self, codegen::generate_asm},
    CompilerResult,
};
use emulator::{config::ROM_DEPTH, run::run};
use serde::Serialize;
use std::{fs, path::Path};
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
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
    /// Run machine code on emulator
    Run { file_path: Option<String> },
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
            let compiler_result = CompilerResult {
                jack_compiler_result,
                vm_compiler_result,
            };
            let json = serde_json::to_string_pretty(&compiler_result).expect("failed to serialize jack compiler result");
            fs::write(debug_output_path, json).expect("failed to write result to debug output path");
            let machine_code = assemble(compiler_result.vm_compiler_result.instructions, ROM_DEPTH);
            fs::write(dest_path, machine_code.join("\n")).expect("failed to write result to dest path");
        }
        Commands::Assemble {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            println!("assembling {} to {}", source_path, dest_path);
            assemble_file(Path::new(source_path), Path::new(dest_path), emulator::config::ROM_DEPTH);
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
        Commands::Run { file_path: file_path_maybe } => {
            let file_path = file_path_maybe.as_ref().expect("path is required");
            println!("running {}", file_path);
            run(file_path);
        }
    }
}
