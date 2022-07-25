mod compilers;
mod emulator;
mod fonts;

use clap::{Parser, Subcommand};
use compilers::{
    assembler::assemble_file,
    jack_compiler::{self, parser::Class},
    vm_compiler,
};
use emulator::run::run;
use fonts::glyphs_class;
use serde::Serialize;
use std::{
    fs,
    io::{self, Read},
    path::Path,
};
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

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Assemble {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            println!("assembling {} to {}", source_path, dest_path);
            assemble_file(
                Path::new(source_path),
                Path::new(dest_path),
                emulator::config::ROM_DEPTH,
            );
        }
        Commands::Compile {
            source_path: source_path_maybe,
            dest_path: dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            println!("assembling {} to {}", source_path, dest_path);
            vm_compiler::compile(Path::new(source_path), Path::new(dest_path)).unwrap();
        }
        Commands::Run {
            file_path: file_path_maybe,
        } => {
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
            let source_path = source_path_maybe
                .as_ref()
                .unwrap_or_else(|| panic!("source path is required"));
            let source = fs::read_to_string(source_path).unwrap();
            let jack_class = jack_compiler::parser::parse(&source);
            let parser_viz_data = ParserVizData {
                parsed_class: jack_class,
                source,
            };
            let json = serde_json::to_string_pretty(&parser_viz_data).unwrap();
            let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../parser_viz_data.json");
            fs::write(out_path, json).unwrap()
        }
    }
}
