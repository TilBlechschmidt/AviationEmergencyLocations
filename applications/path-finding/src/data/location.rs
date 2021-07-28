use std::path::PathBuf;

use geo::{
    point,
    prelude::{Bearing, HaversineDestination, HaversineDistance},
    Point,
};
use uom::si::{angle::degree, f64::*, length::meter};

struct LocationCoordinates {
    start: [f64; 2],
    end: [f64; 2],
}

struct Asset {
    file: PathBuf,
    location: [f64; 2],
}

pub struct Location {
    name: String,

    // Whether or not the runway may be used in both directions
    reversible: bool,

    // Start and end coordinates of the location
    coordinates: LocationCoordinates,

    // List of assets related to the location
    assets: Vec<Asset>,
    // TODO Record potential hazards like people or power lines
}

impl Location {
    // Beginning of the usable runway surface
    pub fn start(&self) -> Point<f64> {
        point!(x: self.coordinates.start[0], y: self.coordinates.start[1])
    }

    // End of the usable runway surface
    pub fn end(&self) -> Point<f64> {
        point!(x: self.coordinates.end[0], y: self.coordinates.end[1])
    }

    // Usable length of the runway
    pub fn length(&self) -> Length {
        Length::new::<meter>(self.start().haversine_distance(&self.end()))
    }

    // Bearing of runway (where 0º is North and 90º is East)
    pub fn bearing(&self) -> Angle {
        Angle::new::<degree>(self.start().bearing(self.end()))
    }

    // Whether or not the runway is usable in both directions
    pub fn reversible(&self) -> bool {
        self.reversible
    }
}
