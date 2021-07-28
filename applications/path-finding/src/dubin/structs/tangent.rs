use super::*;
use crate::{Distance, Location};
use geo::prelude::EuclideanDistance;
use uom::si::length::meter;

#[derive(Debug, Clone, Copy, Default)]
pub struct Tangent {
    pub(in super::super) start: Point,
    pub(in super::super) end: Point,
}

impl Tangent {
    pub(in super::super) fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub(in super::super) fn raw_length(&self) -> f64 {
        self.start.euclidean_distance(&self.end)
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
