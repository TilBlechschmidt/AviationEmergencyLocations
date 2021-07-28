use std::fmt::Display;

use uom::si::length::meter;

use crate::{Distance, Location};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub(in super::super) fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub(in super::super) center: Point,
    pub(in super::super) radius: f64,
    pub direction: Direction,
}

impl Circle {
    pub fn center(&self) -> Location {
        (
            Distance::new::<meter>(self.center.x()),
            Distance::new::<meter>(self.center.y()),
        )
    }

    pub fn radius(&self) -> Distance {
        Distance::new::<meter>(self.radius)
    }

    pub(in super::super) fn arc(self, from: Point, to: Point) -> Arc {
        Arc::new(from, to, self)
    }

    pub(in super::super) fn tangent_to(&self, other: &Circle) -> Option<Tangent> {
        use Direction::*;

        // Most of this function was _ahem_ "inspired" by the implementation over here:
        // https://github.com/gieseanw/Dubins/blob/8b901aaecaac0d90842e69a48136008a19064339/Includes.cpp#L16
        // Albeit some parts have been stripped as the circles always have the same radius in our application.
        assert_eq!(
            self.radius, other.radius,
            "Tangents between circles with different radii are not supported"
        );
        // ^ Better to be safe than sorry 🙈

        // General prep-work
        let connection_vector = other.center - self.center;
        let connection_length =
            (connection_vector.x().powi(2) + connection_vector.y().powi(2)).sqrt();
        let connection_unit_vector = connection_vector / connection_length;

        // Determine which tangent to draw
        let mut sign1 = if self.direction == other.direction {
            // Search for an outer tangent when both circles turn the same way
            1.0
        } else {
            // Search for an inner tangent when the circles turn in opposite directions
            -1.0
        };

        let sign2 = if self.direction == Left { 1.0 } else { -1.0 };

        // Preparations independent of `sign2`
        let mut c = (self.radius - sign1 * other.radius) / connection_length;

        // If the two circles are so close that inner tangents can not exist, fall back to the outer tangent
        if c * c > 1.0 {
            sign1 = 1.0;
            c = (self.radius + sign1 * other.radius) / connection_length;
        }

        // If the outer tangent still produces no sensible result, then there are no valid tangents
        if c * c > 1.0 {
            return None;
        }

        let h = (1.0 - c * c).max(0.0).sqrt();

        // Tangent calculations based on `sign2`
        let normal_x = connection_unit_vector.x() * c - sign2 * h * connection_unit_vector.y();
        let normal_y = connection_unit_vector.y() * c + sign2 * h * connection_unit_vector.x();

        let start = Point::new(
            self.center.x() + self.radius * normal_x,
            self.center.y() + self.radius * normal_y,
        );
        let end = Point::new(
            other.center.x() + sign1 * other.radius * normal_x,
            other.center.y() + sign1 * other.radius * normal_y,
        );

        // // Until we know a better way, just use atan2 for the end_angle 🤷‍♂️
        // let start_angle = c.acos() * sign2;
        // let end_angle = (end.y() - other.center.y()).atan2(end.x() - other.center.x());

        Some(Tangent::new(start, end))
    }
}
