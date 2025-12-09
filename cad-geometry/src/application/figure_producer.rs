use rand::Rng;

use crate::figures::{Circle, Figure};

pub struct GeometricFigureProducer {}

impl GeometricFigureProducer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn produce(&self, amount: u32) -> Vec<Box<dyn Figure>> {
        let mut rng = rand::rng();

        let mut items: Vec<Box<dyn Figure>> = Vec::with_capacity(amount as usize);

        for _n in 0..amount {
            let circle = Box::new(Circle::new(rng.random_range(0.0..1e6)));
            items.push(circle);
        }

        items
    }
}
