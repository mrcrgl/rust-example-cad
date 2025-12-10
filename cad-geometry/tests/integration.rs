use cad_geometry::application::figure_producer::GeometricFigureProducer;

#[test]
fn figure_producer() {
    let producer = GeometricFigureProducer::new();

    let items = producer.produce(100);

    assert_eq!(items.len(), 100);
}

#[tokio::test]
async fn figure_producer_async() {
    let producer = GeometricFigureProducer::new();

    let items = producer.produce_async(100).await;

    assert_eq!(items.len(), 100);
}
