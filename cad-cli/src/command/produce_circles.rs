use cad_geometry::application::figure_producer::GeometricFigureProducer;

use crate::error::CliError;

const MAX_AMOUNT: u32 = 1000000;

pub fn produce_circles(amount: u32, json: bool) -> Result<(), CliError> {
    if amount >= MAX_AMOUNT {
        return Err(CliError::User(format!(
            "amount must be smaller or equal to {MAX_AMOUNT}"
        )));
    }

    let producer = GeometricFigureProducer::new();
    let items = producer.produce_circles(amount);

    if json {
        // TODO
        let out = serde_json::to_string_pretty(&items)?;
        println!("{out}");
    } else {
        for item in items {
            println!("Circle: {}", item.radius());
        }
    }

    Ok(())
}
