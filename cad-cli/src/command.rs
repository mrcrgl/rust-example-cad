mod produce_areas;
mod produce_circles;
use std::time::SystemTime;

use crate::{cli::CliCommand, error::CliError};

pub async fn run_command(cmd: CliCommand) -> Result<(), CliError> {
    let start = SystemTime::now();

    match cmd {
        CliCommand::ProduceAreas { amount } => produce_areas::produce_areas(amount),
        CliCommand::ProduceAreasAsync { amount } => {
            produce_areas::produce_areas_async(amount).await
        }
        CliCommand::ProduceCircles { amount, json } => {
            produce_circles::produce_circles(amount, json)?
        }
    }

    println!(
        "Took {}s",
        start.elapsed().unwrap_or_default().as_secs_f32()
    );

    Ok(())
}
