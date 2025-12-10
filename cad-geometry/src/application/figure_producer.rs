use std::sync::Arc;

use futures::future::join_all;
use rand::Rng;

use crate::figures::{Circle, Figure, Rectangle};

pub struct GeometricFigureProducer {
    distribution: f64,
}

impl Default for GeometricFigureProducer {
    fn default() -> Self {
        Self::new()
    }
}

impl GeometricFigureProducer {
    pub fn new() -> Self {
        Self { distribution: 0.5 }
    }

    pub fn produce(&self, amount: u32) -> Vec<Arc<dyn Figure>> {
        let mut items: Vec<Arc<dyn Figure>> = Vec::with_capacity(amount as usize);

        for _n in 0..amount {
            let circle = build_random_figure(self.distribution);
            items.push(circle);
        }

        items
    }

    pub fn produce_circles(&self, amount: u32) -> Vec<Circle> {
        let mut items = Vec::with_capacity(amount as usize);
        for _ in 0..amount {
            items.push(build_random_circle());
        }
        items
    }

    pub async fn produce_async(&self, amount: u32) -> Vec<Arc<dyn Figure>> {
        let mut futs = Vec::with_capacity(amount as usize);

        for _n in 0..amount {
            let distribution = self.distribution;

            let fut = async move { build_random_figure(distribution) };

            futs.push(fut);
        }

        join_all(futs).await
    }
}

fn build_random_figure(distribution: f64) -> Arc<dyn Figure> {
    let mut rng = rand::rng();

    if rng.random_bool(distribution) {
        Arc::new(build_random_circle())
    } else {
        Arc::new(build_random_rectangle())
    }
}

fn build_random_circle() -> Circle {
    let mut rng = rand::rng();
    Circle::new(rng.random_range(0.0..1e6))
}

fn build_random_rectangle() -> Rectangle {
    let mut rng = rand::rng();
    Rectangle::new(rng.random_range(0.0..1e6), rng.random_range(0.0..1e6))
}
