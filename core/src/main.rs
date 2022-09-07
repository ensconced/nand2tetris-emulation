mod compilers;
mod emulator;
mod fonts;

use clap::{Parser, Subcommand};
use compilers::{
    assembler::assemble_file,
    jack_compiler::{
        self,
        codegen::generate_vm_code,
        jack_node_types::Class,
        parser::parse,
        sourcemap::SourceMap,
        tokenizer::{token_defs, TokenKind},
    },
    utils::tokenizer::{Token, Tokenizer},
    vm_compiler,
};
use emulator::run::run;
use fonts::glyphs_class;
use serde::Serialize;
use std::{fs, path::Path};
use ts_rs::TS;

use crate::compilers::jack_compiler::compile_file;

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
    /// Compile jack code to machine code
    CompileJack {
        source_path: Option<String>,
        dest_path: Option<String>,
    },
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
    /// Generate JSON for visualisation of jack parser output
    JackParserViz { source_path: Option<String> },
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
struct DebugOutput {
    tokens: Vec<Token<TokenKind>>,
    sourcemap: SourceMap,
    vm_commands: Vec<String>,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::CompileJack {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            compile_file(Path::new(source_path), Path::new(dest_path)).unwrap()
        }
        Commands::DebugCompile {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            let source = fs::read_to_string(source_path).expect("failed to read source file");
            let mut sourcemap = SourceMap::new();
            let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(&source);
            let filename = Path::new("test");
            let class = parse(filename, &tokens, &mut sourcemap);
            let vm_commands: Vec<_> = generate_vm_code(filename, class, &mut sourcemap)
                .into_iter()
                .map(|cmd| cmd.to_string())
                .collect();
            let debug_output = DebugOutput {
                tokens,
                sourcemap,
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
        Commands::JackParserViz {
            source_path: source_path_maybe,
        } => {
            let source_path = Path::new(source_path_maybe.as_ref().unwrap_or_else(|| panic!("source path is required")));
            let source = fs::read_to_string(source_path).unwrap();
            let mut sourcemap = SourceMap::new();
            let tokens: Vec<_> = Tokenizer::new(token_defs()).tokenize(&source);
            let parser_output = jack_compiler::parser::parse(source_path, &tokens, &mut sourcemap);
            let parser_viz_data = ParserVizData {
                parsed_class: parser_output,
                source,
            };
            let json = serde_json::to_string_pretty(&parser_viz_data).unwrap();
            let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../parser_viz_data.json");
            fs::write(out_path, json).unwrap()
        }
    }
}
