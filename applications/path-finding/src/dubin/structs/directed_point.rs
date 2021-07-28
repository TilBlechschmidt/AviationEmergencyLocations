use super::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct DirectedPoint {
    pub point: Point,
    pub angle: f64,
}

impl Deref for DirectedPoint {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}
