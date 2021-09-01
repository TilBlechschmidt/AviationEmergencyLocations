use crate::{Aircraft, Location};
use js_sys::{Array, Map};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LocationMap(HashMap<String, Location>);

#[wasm_bindgen]
pub struct Parser {}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen(js_name = parseAircrafts)]
    pub fn parse_aircrafts(&mut self, yaml: &str) -> Result<Map, JsValue> {
        let mut aircrafts =
            serde_yaml::from_str::<Vec<Aircraft>>(yaml).map_err(|e| e.to_string())?;

        aircrafts.sort_by_cached_key(|a| a.raw_mtow);

        let map = aircrafts.into_iter().fold(Map::new(), |map, aircraft| {
            map.set(&aircraft.id().into(), &JsValue::from(aircraft));
            map
        });

        Ok(map)
    }

    #[wasm_bindgen(js_name = parseLocations)]
    pub fn parse_locations(&mut self, yaml: &str) -> Result<LocationMap, JsValue> {
        let locations = serde_yaml::from_str::<Vec<Location>>(yaml).map_err(|e| e.to_string())?;
        let map = locations
            .into_iter()
            .map(|location| (location.id(), location))
            .collect();

        Ok(LocationMap(map))
    }
}

#[wasm_bindgen]
impl LocationMap {
    pub fn keys(&self) -> Array {
        self.0.keys().into_iter().map(JsValue::from).collect()
    }

    pub fn get(&self, id: String) -> Option<Location> {
        self.0.get(&id).cloned()
    }
}

impl LocationMap {
    pub fn locations(&self) -> impl Iterator<Item = &Location> {
        self.0.values().into_iter()
    }
}
