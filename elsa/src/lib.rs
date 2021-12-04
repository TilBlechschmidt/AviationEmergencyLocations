use wasm_bindgen::prelude::*;

mod calculator;
mod data;
mod dubin;
mod helpers;

pub use calculator::*;
pub use data::*;

#[wasm_bindgen(start)]
pub fn startup() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("error initializing log");
}
