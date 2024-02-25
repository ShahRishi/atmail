// main.rs
mod commands;

use clap::{Args, Parser, Subcommand};
use dirs;
use std::fs;
use std::path::PathBuf;
use commands::{config, send};


// define OS
#[cfg(target_os = "linux")]
fn get_os() -> u8 {
    return 0
}

#[cfg(target_os = "macos")]
fn get_os() -> u8 {
    return 1
}

#[cfg(target_os = "windows")]
fn get_os() -> u8 {
    return 2
}


// Shared bin controller
#[derive(Parser)]
#[command(version = "1.0", author = "Rishi Shah <shahrishi108@gmail.com>", about = "Send files through your CL")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    // define OS-dependent config directory path
    let os = get_os();

    let path: PathBuf = match os {
        0 => dirs::config_dir().unwrap().join("atmail"),
        1 => dirs::home_dir().unwrap().join("Library/Application Support/atmail"),
        2 => dirs::config_dir().unwrap().join("atmail"),
        _ => dirs::config_dir().unwrap().join("atmail"),
    };

    // create config directory
    match fs::create_dir_all(path.clone()) {
        Ok(_) => {},
        Err(e) => println!("Error creating config directory: {}", e)
    }

    // define commands
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Send(send_args) => {
            send(send_args.to.clone(), send_args.path.clone(), &path);
        }

        Commands::Config(config_args) => {
            config(config_args.email.clone(), config_args.token.clone(), &path);
        }
    }
}