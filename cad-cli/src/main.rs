use std::process::exit;

use clap::Parser;

use crate::{cli::Cli, command::run_command};

mod cli;
mod command;
mod error;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match run_command(command).await {
            Ok(_) => exit(0),
            Err(err) => {
                eprintln!("error: {err}");
                exit(err.exit_code());
            }
        }
    }
}
