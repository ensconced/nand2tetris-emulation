mod assembler;
mod computer;
mod display;
mod programmer;
mod run;

use assembler::assemble_file;
use clap::{Parser, Subcommand};
use run::run;

/// Simple program to greet a person
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
            assemble_file(source_path, dest_path, rom_depth);
        }
        Commands::Run { file_path_maybe } => {
            let file_path = file_path_maybe.as_ref().expect("path is required");
            println!("running {}", file_path);
            run(file_path);
        }
    }
}
