mod produce_areas;
use crate::cli::CliCommand;

pub fn run_command(cmd: CliCommand) {
    match cmd {
        CliCommand::ProduceAreas { amount } => produce_areas::produce_areas(amount),
    }
}
