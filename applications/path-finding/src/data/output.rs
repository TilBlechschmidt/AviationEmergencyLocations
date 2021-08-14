use super::{
    Aircraft, AircraftIdentifier, GlidePerformance, LandingPerformance, Location,
    RiskClassification, SurfaceType, TakeoffPerformance,
};
use crate::Simulation;
use geojson::GeoJson;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use uom::si::{
    angle::degree,
    f64::{Angle, Length},
    length::{foot, meter},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub aircrafts: Vec<DerivedAircraft>,
    pub locations: Vec<DerivedLocation>,
    pub parameters: SimulationParameters,
}

impl Output {
    pub fn new(aircrafts: Vec<Aircraft>, locations: Vec<Location>, simulation: Simulation) -> Self {
        let parameters = SimulationParameters::new(&simulation);

        let locations = locations
            .into_par_iter()
            .map(|location| DerivedLocation::new(location, &aircrafts, &simulation))
            .collect();

        let aircrafts = aircrafts
            .into_iter()
            .map(|aircraft| DerivedAircraft::new(aircraft, simulation.bank))
            .collect();

        Self {
            aircrafts,
            locations,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationParameters {
    /// Angle of bank during turns (degrees)
    pub bank_angle: usize,

    /// Spatial resolution of all calculations (meters)
    pub resolution: usize,

    /// Altitudes for which range profiles have been generated (feet)
    /// The values from this array may be used as keys for the ranges map of the [`DerivedLocation`] struct.
    pub altitudes: Vec<usize>,
}

impl SimulationParameters {
    pub fn new(simulation: &Simulation) -> Self {
        Self {
            bank_angle: convert_angle(simulation.bank),
            resolution: simulation.resolution.get::<meter>().round() as usize,
            altitudes: simulation
                .altitudes
                .iter()
                .map(|a| a.get::<foot>().round() as usize)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedAircraft {
    #[serde(flatten)]
    pub aircraft: Aircraft,

    /// Performance numbers that have been derived from the official information
    pub derived_performance: DerivedAircraftPerformance,
}

impl DerivedAircraft {
    pub fn new(aircraft: Aircraft, bank: Angle) -> Self {
        Self {
            derived_performance: DerivedAircraftPerformance::new(&aircraft, bank),
            aircraft,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedAircraftPerformance {
    pub glide: DerivedAircraftGlidePerformance,
    pub landing: DerivedAircraftLandingPerformance,
}

impl DerivedAircraftPerformance {
    pub fn new(aircraft: &Aircraft, bank: Angle) -> Self {
        Self {
            glide: DerivedAircraftGlidePerformance::new(&aircraft.glide, bank),
            landing: DerivedAircraftLandingPerformance::new(&aircraft.landing),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedAircraftLandingPerformance {
    /// Landing distance to clear a 50ft obstable per surface type (meters)
    pub distance_on_surface: HashMap<SurfaceType, usize>,
}

impl DerivedAircraftLandingPerformance {
    pub fn new(landing: &LandingPerformance) -> Self {
        Self {
            distance_on_surface: SurfaceType::iter()
                .map(|surface| {
                    (
                        surface,
                        landing
                            .total_distance_on_surface(&surface)
                            .get::<meter>()
                            .round() as usize,
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedAircraftGlidePerformance {
    /// Turn radius during glide in a 45º bank (meters)
    pub turn_radius: usize,

    /// Height lost in a 360º turn at 45º bank (feet)
    pub full_turn_height_loss: usize,
}

impl DerivedAircraftGlidePerformance {
    pub fn new(glide: &GlidePerformance, bank: Angle) -> Self {
        Self {
            turn_radius: glide.turn_radius(bank).get::<meter>().round() as usize,
            full_turn_height_loss: glide
                .height_lost_for_turn(bank, Angle::new::<degree>(360.0))
                .get::<foot>()
                .round() as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedLocation {
    #[serde(flatten)]
    pub location: Location,

    /// Usable length of the runway
    pub length: usize,

    /// Bearing of runway (degrees where 0º is North and 90º is East)
    pub bearing: usize,

    /// Bearing of the reverse direction
    pub reverse_bearing: Option<usize>,

    /// Geographic range polygons for each aircraft and altitude (in feet)
    pub ranges: HashMap<AircraftIdentifier, HashMap<usize, GeoJson>>,

    /// Risk classifications for each aircraft
    pub risks: HashMap<AircraftIdentifier, RiskClassification>,
}

impl DerivedLocation {
    pub fn new(location: Location, aircrafts: &Vec<Aircraft>, simulation: &Simulation) -> Self {
        let ranges = aircrafts
            .par_iter()
            .map(|aircraft| {
                (
                    aircraft.id,
                    simulation.generate_range_profiles(&location, aircraft, 0.75),
                )
            })
            .collect();

        let risks = aircrafts
            .iter()
            .map(|aircraft| (aircraft.id, location.risk(&aircraft)))
            .collect();

        let reverse_bearing = if location.reversible {
            Some(convert_angle(location.reverse_bearing()))
        } else {
            None
        };

        Self {
            length: location.length().get::<meter>().round() as usize,
            bearing: convert_angle(location.bearing()),
            reverse_bearing,
            ranges,
            risks,
            location,
        }
    }
}

fn convert_angle(angle: Angle) -> usize {
    let mut theta = angle.get::<degree>();

    if theta < 0.0 {
        theta += 360.0;
    }

    assert!(theta >= 0.0);

    theta.round() as usize
}
