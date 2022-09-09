mod compilers;
mod emulator;
mod fonts;

use clap::{Parser, Subcommand};
use compilers::{
    assembler::assemble_file,
    jack_compiler::{compile_jack, jack_node_types::Class},
    vm_compiler,
};
use emulator::run::run;
use fonts::glyphs_class;
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
        source_path: Option<String>,
        dest_path: Option<String>,
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
    /// Generate glyphs stdlib module from fonts file
    GenerateGlyphs,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::DebugCompile {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            let source = fs::read_to_string(source_path).expect("failed to read source file");
            let filename = Path::new("test");
            let jack_compiler_result = compile_jack(filename, source);
            let json = serde_json::to_string_pretty(&jack_compiler_result).expect("failed to serialize jack compiler result");
            fs::write(dest_path, json).expect("failed to write result to dest path");
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
        Commands::GenerateGlyphs => {
            fs::write("./std_lib/Glyphs.jack", glyphs_class()).unwrap();
        }
    }
}
