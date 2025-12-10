use cad_geometry::application::figure_producer::GeometricFigureProducer;

pub fn produce_areas(amount: u32) {
    println!("todo produce {amount} areas");

    let producer = GeometricFigureProducer::new();
    let items = producer.produce(amount);

    for item in items {
        println!("{}({:?})", item.figure_type(), item.to_area())
    }
}

pub async fn produce_areas_async(amount: u32) {
    let producer = GeometricFigureProducer::new();
    let items = producer.produce_async(amount).await;

    for item in &items {
        println!("{}({:?})", item.figure_type(), item.to_area())
    }
}
