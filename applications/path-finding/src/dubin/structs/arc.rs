use super::*;
use crate::{Distance, Location};
use std::f64::consts::PI;
use uom::si::{angle::radian, f64::Angle, length::meter};

#[derive(Debug)]
pub struct Arc {
    pub circle: Circle,
    pub(in super::super) start: Point,
    pub(in super::super) end: Point,
}

impl Arc {
    pub(in super::super) fn new(from: Point, to: Point, circle: Circle) -> Self {
        Self {
            circle,
            start: from,
            end: to,
        }
    }

    pub(in super::super) fn raw_angle(&self) -> f64 {
        let start_vec = self.start - self.circle.center;
        let end_vec = self.end - self.circle.center;

        let mut theta = end_vec.y().atan2(end_vec.x()) - start_vec.y().atan2(start_vec.x());

        if theta < -1e-6 && self.circle.direction == Direction::Right {
            theta += 2.0 * PI;
        } else if theta > 1e-6 && self.circle.direction == Direction::Left {
            theta -= 2.0 * PI;
        }

        theta
    }

    pub(in super::super) fn raw_length(&self) -> f64 {
        (self.raw_angle() * self.circle.radius).abs()
    }

    pub fn start_angle(&self) -> Angle {
        let end_vec = self.end - self.circle.center;
        let mut theta = end_vec.y().atan2(end_vec.x());

        if theta < -1e-6 && self.circle.direction == Direction::Right {
            theta += 2.0 * PI;
        } else if theta > 1e-6 && self.circle.direction == Direction::Left {
            theta -= 2.0 * PI;
        }

        Angle::new::<radian>(theta)
    }

    pub fn angle(&self) -> Angle {
        Angle::new::<radian>(self.raw_angle())
    }

    pub fn length(&self) -> Distance {
        Distance::new::<meter>(self.raw_length())
    }

    pub fn start(&self) -> Location {
        (
            Distance::new::<meter>(self.start.x()),
            Distance::new::<meter>(self.start.y()),
        )
    }

    pub fn end(&self) -> Location {
        (
            Distance::new::<meter>(self.end.x()),
            Distance::new::<meter>(self.end.y()),
        )
    }
}
