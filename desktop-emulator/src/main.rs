mod io;
mod run;

use run::run;
use std::fs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run machine code on emulator
    Run { file_path: Option<String> },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Run { file_path: file_path_maybe } => {
            let file_path = file_path_maybe.as_ref().expect("path is required");
            run(fs::read_to_string(file_path).expect("failed to read machine code from file"));
        }
    }
}
