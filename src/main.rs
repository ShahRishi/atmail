// main.rs
mod commands;

use clap::{Args, Parser, Subcommand};
use commands::{send};


// Shared bin controller
#[derive(Parser)]
#[command(version = "1.0", author = "Rishi Shah <shahrishi108@gmail.com>", about = "Send files through your CL")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send your file
    Send,
}


fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Send => {
            send();
        }
    }
}