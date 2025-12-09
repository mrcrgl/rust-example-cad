use std::f32::consts::PI;

use crate::{area::ToArea, figures::Figure};

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Figure for Circle {
    fn figure_type(&self) -> super::FigureType {
        super::FigureType::Circle
    }
}

impl ToArea for Circle {
    fn to_area(&self) -> f32 {
        self.radius * self.radius * PI
    }
}
