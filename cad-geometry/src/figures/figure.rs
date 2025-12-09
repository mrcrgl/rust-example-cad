use std::fmt::Display;

use crate::area::ToArea;

const FIGURE_TYPE_CIRCLE: &str = "circle";
const FIGURE_TYPE_RECTANGLE: &str = "rectangle";

pub enum FigureType {
    Circle,
    Rectangle,
    Other(String),
}

impl Display for FigureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureType::Circle => write!(f, "{FIGURE_TYPE_CIRCLE}"),
            FigureType::Rectangle => write!(f, "{FIGURE_TYPE_RECTANGLE}"),
            FigureType::Other(other) => write!(f, "{other}"),
        }
    }
}

pub trait Figure: ToArea {
    fn figure_type(&self) -> FigureType;
}
