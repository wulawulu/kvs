use clap::Parser;
use kvs::{Cli, Commands};
use std::process::exit;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Set { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Remove { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
