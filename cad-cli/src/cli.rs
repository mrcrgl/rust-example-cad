use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Subcommand)]
pub enum CliCommand {
    ProduceAreas {
        #[arg(short, long, default_value_t = 1)]
        amount: u32,
    },
    ProduceAreasAsync {
        #[arg(short, long, default_value_t = 1)]
        amount: u32,
    },
    ProduceCircles {
        #[arg(short, long, default_value_t = 1)]
        amount: u32,

        #[arg(long, default_value_t = false)]
        json: bool,
    },
}
