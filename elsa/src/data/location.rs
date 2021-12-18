use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Add;

use geo::geodesic_distance::GeodesicDistance;
use geo::prelude::{Bearing, BoundingRect, Centroid, HaversineDestination};
use geo::{point, Line, LineString, Point, Polygon, Rect};
use js_sys::Array;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use wasm_bindgen::prelude::*;

use crate::Aircraft;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SerializedCoordinate([f64; 2]);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LocationCoordinates {
    start: SerializedCoordinate,
    end: SerializedCoordinate,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum SurfaceType {
    Asphalt = "Asphalt",
    Gras = "Gras",
    Water = "Water",
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum UsageType {
    Agricultural = "Agricultural",
    Aeronautical = "Aeronautical",
    Nature = "Nature",
    Waterway = "Waterway",
    Event = "Event",
    Park = "Park",
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum HumanPresenceCategory {
    /// During daylight and especially with good weather strong human presence is to be expected, potentially making the location unviable
    Dense = "Dense",
    /// Humans may occasionally be present but are usually spread out and on the move, it is usually possible to find a landing path without any in the way
    Sparse = "Sparse",
    /// Generally no humans are on-site unless an event is taking place
    EventOnly = "EventOnly",
    /// It is not likely that humans will ever pose a risk at this location
    Unlikely = "Unlikely",
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum RiskClassification {
    /// Sufficient landing distance available, no hazards, no humans, and no major expected damage to the aircraft
    Safe = "Safe",
    /// Potential damage to the aircraft or bystanders due to short landing distance or human presence
    Risky = "Risky",
    /// Guaranteed damage to the aircraft, questionable outcome for the passengers, high likelyhood for outside damage
    Unsafe = "Unsafe",
}

impl Into<u8> for RiskClassification {
    fn into(self) -> u8 {
        match self {
            RiskClassification::Safe => 0,
            RiskClassification::Risky => 1,
            RiskClassification::Unsafe => 2,
            _ => panic!("unexpected risk classification variant"),
        }
    }
}

impl From<u8> for RiskClassification {
    fn from(value: u8) -> Self {
        match value {
            0 => RiskClassification::Safe,
            1 => RiskClassification::Risky,
            2 => RiskClassification::Unsafe,
            _ => panic!("Not a valid risk classification: {}", value),
        }
    }
}

impl Add for RiskClassification {
    type Output = RiskClassification;

    fn add(self, rhs: Self) -> Self::Output {
        let a: u8 = self.into();
        let b: u8 = rhs.into();

        a.max(b).into()
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[wasm_bindgen(skip)]
    pub name: String,

    /// Elevation above MSL in meters
    // pub elevation: usize,

    /// Whether or not the runway may be used in both directions
    #[wasm_bindgen(readonly)]
    pub reversible: bool,

    /// Kind of surface at the site
    #[wasm_bindgen(readonly)]
    pub surface: SurfaceType,

    /// Whether humans could be present that may or may not give way
    #[wasm_bindgen(readonly, js_name = "humanPresence")]
    #[serde(default)]
    pub human_presence: HumanPresenceCategory,

    /// What the location is used for
    #[wasm_bindgen(readonly)]
    pub usage: UsageType,

    /// Field elevation above MSL in feet
    #[wasm_bindgen(readonly)]
    pub elevation: u32,

    /// Start and end coordinates of the location
    coordinates: LocationCoordinates,

    /// Date at which the data was last verified
    #[wasm_bindgen(skip)]
    pub survey_date: String,

    /// Human-readable remarks about the location
    #[wasm_bindgen(skip)]
    pub remarks: Option<String>,
    //
    // TODO Record other potential hazards like power lines or tents
    // TODO Add free-form text and web links (e.g. event calendar)
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

    /// Line representing the landable surface
    pub fn line(&self) -> Line<f64> {
        Line::new(self.start(), self.end())
    }

    /// Center of the runway
    pub fn centroid(&self) -> Point<f64> {
        self.line().centroid()
    }

    pub fn bounding_rect(&self) -> Rect<f64> {
        self.line().bounding_rect()
    }

    pub fn spaced_polygon(&self, distance: f64) -> Polygon<f64> {
        let start = self.start();
        let end = self.end();
        let bearing = self.start().bearing(self.end());
        let reverse_bearing = self.end().bearing(self.start());

        let a = start.haversine_destination(reverse_bearing + 45.0, distance);
        let b = start.haversine_destination(reverse_bearing - 45.0, distance);

        let c = end.haversine_destination(bearing + 45.0, distance);
        let d = end.haversine_destination(bearing - 45.0, distance);

        Polygon::new(LineString::from(vec![a, b, c, d]), vec![])
    }

    /// Returns a number of points with a given resolution in meters where a given airplane can land
    pub fn landable_points(&self, aircraft: &Aircraft, resolution: f64) -> Vec<(Point<f64>, f64)> {
        let inset_at_ends =
            self.length() - aircraft.landing.total_distance_on_surface(&self.surface);

        let mut points = vec![(self.start(), self.bearing())];

        // Add the other start point if applicable
        if self.reversible {
            points.push((self.end(), self.reverse_bearing()));
        }

        // Iterate over the insets at each end
        if inset_at_ends > 0.0 {
            let bearing = self.bearing();
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
                        .haversine_destination(self.reverse_bearing(), step_distance);
                    points.push((inset_from_end, self.reverse_bearing()));
                }
            }
        }

        points
    }
}

#[wasm_bindgen]
impl Location {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        let mut hasher = DefaultHasher::new();

        let start = self.coordinates.start.point();
        hasher.write(&start.x().to_le_bytes());
        hasher.write(&start.y().to_le_bytes());

        let end = self.coordinates.end.point();
        hasher.write(&end.x().to_le_bytes());
        hasher.write(&end.y().to_le_bytes());

        format!("{:x}", hasher.finish())
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Human-readable remarks about the location
    #[wasm_bindgen(getter)]
    pub fn remarks(&self) -> Option<String> {
        self.remarks.clone()
    }

    /// Date at which the data was last verified
    #[wasm_bindgen(getter, js_name = "surveyDate")]
    pub fn survey_date(&self) -> String {
        self.survey_date.clone()
    }

    /// Coordinates of the runway
    #[wasm_bindgen(getter)]
    pub fn coordinates(&self) -> Array {
        let start_array = Array::new();
        start_array.push(&self.coordinates.start.longitude().into());
        start_array.push(&self.coordinates.start.latitude().into());

        let end_array = Array::new();
        end_array.push(&self.coordinates.end.longitude().into());
        end_array.push(&self.coordinates.end.latitude().into());

        let coordinate_array = Array::new();
        coordinate_array.push(&start_array);
        coordinate_array.push(&end_array);

        coordinate_array
    }

    /// Usable length of the runway
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> f64 {
        self.start().geodesic_distance(&self.end())
    }

    /// Bearing of runway in degrees (where 0º is North and 90º is East)
    #[wasm_bindgen(getter)]
    pub fn bearing(&self) -> f64 {
        self.start().bearing(self.end())
    }

    /// Same as bearing but for the reverse direction
    #[wasm_bindgen(getter, js_name = "reverseBearing")]
    pub fn reverse_bearing(&self) -> f64 {
        assert!(
            self.reversible,
            "Attempted to read reverse bearing of a non-reversible runway"
        );
        self.end().bearing(self.start())
    }

    /// Maximum distance from the beginning of the landable surface where the 50ft height has to be reached
    /// in order to have sufficient landing run available to come to a complete stop.
    pub fn inset(&self, aircraft: &Aircraft) -> f64 {
        let distance_required = aircraft.landing.total_distance_on_surface(&self.surface);
        let distance_available = self.length();

        distance_available - distance_required
    }

    /// Fraction of required landing distance that is available in addition to the base 100%
    #[wasm_bindgen(js_name = "landingHeadroom")]
    pub fn landing_headroom(&self, aircraft: &Aircraft) -> f64 {
        let required_landing_distance = aircraft.landing.total_distance_on_surface(&self.surface);
        let remaining_landing_distance = self.length() - required_landing_distance;

        remaining_landing_distance / required_landing_distance
    }
}

impl Default for HumanPresenceCategory {
    fn default() -> Self {
        HumanPresenceCategory::Unlikely
    }
}

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
