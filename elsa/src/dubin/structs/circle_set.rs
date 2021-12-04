use std::f64::consts::FRAC_PI_2;

use super::*;

#[derive(Debug)]
pub struct CircleSet {
    pub start_left: Circle,
    pub start_right: Circle,

    pub end_left: Circle,
    pub end_right: Circle,
}

impl CircleSet {
    pub fn new(start: &DirectedPoint, end: &DirectedPoint, radius: f64) -> Self {
        use Direction::*;

        let start_left_offset = Point::new(
            radius * (start.angle - FRAC_PI_2).cos(),
            radius * (start.angle - FRAC_PI_2).sin(),
        );
        let start_left = Circle {
            center: **start + start_left_offset,
            radius,
            direction: Left,
        };

        let start_right_offset = Point::new(
            radius * (start.angle + FRAC_PI_2).cos(),
            radius * (start.angle + FRAC_PI_2).sin(),
        );
        let start_right = Circle {
            center: **start + start_right_offset,
            radius,
            direction: Right,
        };

        let end_left_offset = Point::new(
            radius * (end.angle - FRAC_PI_2).cos(),
            radius * (end.angle - FRAC_PI_2).sin(),
        );
        let end_left = Circle {
            center: **end + end_left_offset,
            radius,
            direction: Left,
        };

        let end_right_offset = Point::new(
            radius * (end.angle + FRAC_PI_2).cos(),
            radius * (end.angle + FRAC_PI_2).sin(),
        );
        let end_right = Circle {
            center: **end + end_right_offset,
            radius,
            direction: Right,
        };

        Self {
            start_left,
            start_right,
            end_left,
            end_right,
        }
    }
}
