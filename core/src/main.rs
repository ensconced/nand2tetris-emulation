mod compilers;
mod emulator;
mod fonts;

use clap::{Parser, Subcommand};
use compilers::{
    assembler::assemble_file,
    jack_compiler::{
        codegen::{generate_vm_code, JackCodegenResult},
        jack_node_types::Class,
        parser::parse,
        sourcemap::JackCodegenSourceMap,
        tokenizer::{token_defs, TokenKind},
    },
    utils::tokenizer::{Token, Tokenizer},
    vm_compiler,
};
use emulator::run::run;
use fonts::glyphs_class;
use serde::Serialize;
use std::{collections::HashMap, fs, path::Path};
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

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
struct DebugOutput {
    tokens: HashMap<String, Vec<Token<TokenKind>>>,
    sourcemap: JackCodegenSourceMap,
    vm_commands: Vec<String>,
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
            let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(&source);
            let filename = Path::new("test");
            let jack_compile_result = parse(filename, &tokens);
            let JackCodegenResult {
                commands,
                sourcemap: codegen_sourcemap,
            } = generate_vm_code(filename, jack_compile_result.class);
            let vm_commands: Vec<_> = commands.into_iter().map(|cmd| cmd.to_string()).collect();

            let mut tokens_hashmap = HashMap::new();
            tokens_hashmap.insert(source_path.to_owned(), tokens);
            let debug_output = DebugOutput {
                tokens: tokens_hashmap,
                sourcemap: codegen_sourcemap,
                vm_commands,
            };
            let json = serde_json::to_string_pretty(&debug_output).unwrap();
            fs::write(dest_path, json).unwrap();
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
