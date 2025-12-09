mod produce_areas;
use crate::cli::CliCommand;

pub async fn run_command(cmd: CliCommand) {
    match cmd {
        CliCommand::ProduceAreas { amount } => produce_areas::produce_areas(amount),
        CliCommand::ProduceAreasAsync { amount } => {
            produce_areas::produce_areas_async(amount).await
        }
    }
}
