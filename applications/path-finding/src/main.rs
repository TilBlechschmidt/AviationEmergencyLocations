#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{
    data::{Aircraft, Location, Output},
    sim::Simulation,
};
use std::{fs::File, io::Write};
use uom::si::{
    angle::{degree, radian},
    f64::{Angle, Length},
    length::{foot, meter},
};

mod data;
mod dubin;
mod sim;

// mod drawing;

fn main() {
    let raw_aircrafts = std::fs::read_to_string("../../data/aircraft.yml").unwrap();
    let raw_locations = std::fs::read_to_string("../../data/locations.yml").unwrap();

    let aircrafts: Vec<Aircraft> = serde_yaml::from_str(&raw_aircrafts).unwrap();
    let locations: Vec<Location> = serde_yaml::from_str(&raw_locations).unwrap();

    let simulation = Simulation {
        bank: Angle::new::<degree>(45.0),
        resolution: Length::new::<meter>(10.0),
        altitudes: vec![
            Length::new::<foot>(1500.0),
            Length::new::<foot>(2000.0),
            Length::new::<foot>(2300.0),
        ],
    };

    let output = Output::new(aircrafts, locations, simulation);
    let json = serde_json::to_string_pretty(&output).unwrap();
    let mut file = File::create("out/output.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}
