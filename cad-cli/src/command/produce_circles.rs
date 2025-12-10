use cad_geometry::application::figure_producer::GeometricFigureProducer;

pub fn produce_circles(amount: u32, json: bool) {
    let producer = GeometricFigureProducer::new();
    let items = producer.produce_circles(amount);

    if json {
        // TODO
        let out = serde_json::to_string_pretty(&items);
    } else {
        for item in items {
            println!("Circle: {}", item.radius());
        }
    }
}
