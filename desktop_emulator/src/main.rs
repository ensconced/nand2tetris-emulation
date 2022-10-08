use clap::{Parser, Subcommand};
use emulator_core::run::run;

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
            println!("running {}", file_path);
            run(file_path);
        }
    }
}
