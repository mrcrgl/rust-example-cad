use std::f32::consts::PI;

use serde::Serialize;

use crate::{area::ToArea, figures::Figure};

/// Circle figure.
///
/// Representation of a circle.
///
/// # Examples
///
/// ```rust
/// use cad_geometry::figures::Circle;
///
/// let circle = Circle::new(1.3);
///
/// assert_eq!(circle.radius(), 1.3);
/// ```
///
/// # Panics
///
/// ```rust
/// // This code panics! And must!
///
/// ```
#[derive(Debug, Serialize)]
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
    fn to_area(&self) -> Option<f32> {
        let area = self.radius * self.radius * PI;
        if area == f32::INFINITY {
            None
        } else {
            Some(area)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32;

    use super::*;

    #[test]
    fn radius_getter() {
        let value = 32.1;
        let c = Circle::new(value);

        assert_eq!(c.radius(), value);
    }

    #[test]
    fn radius_getter_max() {
        let value = f32::MAX;
        let c = Circle::new(value);

        assert_eq!(c.radius(), value);
    }

    #[test]
    fn to_area_max() {
        let value = f32::MAX;
        let c = Circle::new(value);

        assert_eq!(c.to_area(), None);
    }

    #[test]
    fn to_area() {
        let value = 32.1;
        let c = Circle::new(value);

        assert_eq!(c.to_area(), Some(32.1 * 32.1 * PI));
    }
}
