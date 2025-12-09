use crate::{area::ToArea, figures::Figure};

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Rectangle { width, height }
    }

    pub fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

impl Figure for Rectangle {
    fn figure_type(&self) -> super::FigureType {
        super::FigureType::Rectangle
    }
}

impl ToArea for Rectangle {
    fn to_area(&self) -> f32 {
        self.height * self.width
    }
}
