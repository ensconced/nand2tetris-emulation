mod compilers;
mod computer;
mod io;
mod programmer;
mod run;
mod tokenizer;

use std::path::Path;

use clap::{Parser, Subcommand};
use compilers::assembler::assemble_file;
use run::run;

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
        source_path_maybe: Option<String>,
        dest_path_maybe: Option<String>,
    },
    /// Run machine code on emulator
    Run { file_path_maybe: Option<String> },
}

const ROM_DEPTH: usize = 32768;

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Assemble {
            source_path_maybe,
            dest_path_maybe,
        } => {
            let source_path = source_path_maybe.as_ref().expect("source path is required");
            let dest_path = dest_path_maybe.as_ref().expect("dest path is required");
            println!("assembling {} to {}", source_path, dest_path);
            assemble_file(Path::new(source_path), Path::new(dest_path), ROM_DEPTH);
        }
        Commands::Run { file_path_maybe } => {
            let file_path = file_path_maybe.as_ref().expect("path is required");
            println!("running {}", file_path);
            run(file_path);
        }
    }
}
