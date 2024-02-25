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
    Send(SendArgs),
    Config(ConfigArgs)
}

#[derive(Args)]
struct SendArgs {
    #[arg(name = "to")]
    to: String,

    #[arg(name = "path")]
    path: String
}

#[derive(Args)]
struct ConfigArgs {
    #[arg(name = "email")]
    email: String,

    #[arg(name = "token")]
    token: String
}


fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Send(send_args) => {
            send(send_args.to.clone(), send_args.path.clone());
        }
    }
}