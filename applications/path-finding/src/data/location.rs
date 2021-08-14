use super::Aircraft;
use geo::{
    point,
    prelude::{Bearing, Centroid, GeodesicDistance, GeodesicIntermediate, HaversineDestination},
    Line, Point,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::EnumIter;
use uom::si::{angle::degree, f64::*, length::meter};
use uuid::Uuid;

pub type LocationIdentifier = Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct SerializedCoordinate([f64; 2]);

impl SerializedCoordinate {
    fn latitude(&self) -> f64 {
        self.0[0]
    }

    fn longitude(&self) -> f64 {
        self.0[1]
    }

    fn point(&self) -> Point<f64> {
        // For some reason it is "standard" to flip lat/lon to lon/lat
        point!(x: self.longitude(), y: self.latitude())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocationCoordinates {
    start: SerializedCoordinate,
    end: SerializedCoordinate,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum SurfaceType {
    Asphalt,
    Gras,
    Water,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    file: PathBuf,
    location: SerializedCoordinate,
    bearing: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiskClassification {
    /// Sufficient landing distance available, no hazards, no humans, and no major expected damage to the aircraft
    Safe,
    /// Potential damage to the aircraft or bystanders due to short landing distance or human presence
    Risky,
    /// Guaranteed damage to the aircraft, questionable outcome for the passengers, high likelyhood for outside damage
    Unsafe,
}

fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(default = "new_uuid")]
    pub id: LocationIdentifier,
    pub name: String,

    /// Whether or not the runway may be used in both directions
    pub reversible: bool,

    /// Kind of surface at the site
    pub surface: SurfaceType,

    /// Whether humans could be present that may or may not give way
    // TODO Record other potential hazards like power lines or tents
    #[serde(default)]
    pub human_presence: bool,

    /// Start and end coordinates of the location
    coordinates: LocationCoordinates,

    /// List of assets related to the location
    assets: Vec<Asset>,
}

impl Location {
    /// Beginning of the usable runway surface
    pub fn start(&self) -> Point<f64> {
        self.coordinates.start.point()
    }

    /// End of the usable runway surface
    pub fn end(&self) -> Point<f64> {
        self.coordinates.end.point()
    }

    /// Usable length of the runway
    pub fn length(&self) -> Length {
        Length::new::<meter>(self.start().geodesic_distance(&self.end()))
    }

    /// Bearing of runway (where 0º is North and 90º is East)
    pub fn bearing(&self) -> Angle {
        Angle::new::<degree>(self.start().bearing(self.end()))
    }

    /// Center of the runway
    pub fn centroid(&self) -> Point<f64> {
        Line::new(self.start(), self.end()).centroid()
    }

    /// Same as bearing but for the reverse direction
    pub fn reverse_bearing(&self) -> Angle {
        assert!(
            self.reversible,
            "Attempted to read reverse bearing of a non-reversible runway"
        );
        Angle::new::<degree>(self.end().bearing(self.start()))
    }

    /// Returns a number of points with a given resolution where a given airplane can land
    pub fn landable_points(
        &self,
        aircraft: &Aircraft,
        resolution: Length,
    ) -> Vec<(Point<f64>, Angle)> {
        let resolution = resolution.get::<meter>();
        let inset_at_ends = (self.length()
            - aircraft.landing.total_distance_on_surface(&self.surface))
        .get::<meter>();

        let mut points = vec![(self.start(), self.bearing())];

        // Add the other start point if applicable
        if self.reversible {
            points.push((self.end(), self.reverse_bearing()));
        }

        // Iterate over the insets at each end
        if inset_at_ends > 0.0 {
            let bearing = self.bearing().get::<degree>();
            let reverse_bearing = self.reverse_bearing().get::<degree>();
            let step_count = (inset_at_ends / resolution).floor() as usize;

            for i in 1..(step_count + 1) {
                let step_distance = resolution * (i as f64);

                // Inset from start
                let inset_from_start = self.start().haversine_destination(bearing, step_distance);
                points.push((inset_from_start, self.bearing()));

                // Inset from end
                if self.reversible {
                    let inset_from_end = self
                        .end()
                        .haversine_destination(reverse_bearing, step_distance);
                    points.push((inset_from_end, self.reverse_bearing()));
                }
            }
        }

        points
    }

    pub fn risk(&self, aircraft: &Aircraft) -> RiskClassification {
        let mut risky = false;
        let mut not_safe = false;

        // Check the landing distance
        let required_landing_distance = aircraft.landing.total_distance_on_surface(&self.surface);
        let remaining_landing_distance = self.length() - required_landing_distance;
        risky = risky || remaining_landing_distance.get::<meter>() < 0.0;

        // Check the surface type for destructive properties
        not_safe = not_safe || self.surface == SurfaceType::Water;

        // Check for pesky humans
        risky = risky || self.human_presence;

        if not_safe {
            RiskClassification::Unsafe
        } else if risky {
            RiskClassification::Risky
        } else {
            RiskClassification::Safe
        }
    }
}
