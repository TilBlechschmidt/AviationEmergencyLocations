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
use wasm_bindgen::prelude::*;

mod data;
mod dubin;
mod sim;

#[wasm_bindgen]
pub struct Foo {
    answer: u8,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(answer: u8) -> Foo {
        Foo { answer }
    }

    /// This is some description!
    pub fn answer(&self) -> u8 {
        self.answer
    }
}

#[wasm_bindgen(start)]
pub fn startup() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("error initializing log");
}

#[wasm_bindgen]
pub fn do_stuff() {
    log::info!("Hello world from Rust!");
    // let aircrafts_input = include_str!("../../../data/aircraft.yml");
    // let locations_input = include_str!("../../../data/locations.yml");

    // let aircrafts: Vec<Aircraft> = serde_yaml::from_str(&aircrafts_input).unwrap();
    // let locations: Vec<Location> = serde_yaml::from_str(&locations_input).unwrap();

    // let simulation = Simulation {
    //     bank: Angle::new::<degree>(45.0),
    //     resolution: Length::new::<meter>(10.0),
    //     altitudes: vec![
    //         Length::new::<foot>(1500.0),
    //         Length::new::<foot>(2000.0),
    //         Length::new::<foot>(2300.0),
    //     ],
    // };

    // let output = Output::new(aircrafts, locations, simulation);
    // let json = serde_json::to_string_pretty(&output).unwrap();
    // println!("{}", json);
}
