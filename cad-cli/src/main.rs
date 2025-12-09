use clap::Parser;

use crate::{cli::Cli, command::run_command};

mod cli;
mod command;

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        run_command(command);
    }
}
