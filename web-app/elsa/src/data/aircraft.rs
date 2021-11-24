use crate::{
    dubin::{DubinPath, GeographicDubinPath},
    SurfaceType,
};
use serde::{Deserialize, Serialize};
use uom::si::{
    acceleration::meter_per_second_squared,
    angle::radian,
    f64::{Acceleration, Length, Mass, Velocity},
    length::{foot, meter, nautical_mile},
    mass::{kilogram, pound},
    velocity::{foot_per_minute, knot, meter_per_second},
};
use wasm_bindgen::prelude::*;

const TURN_AIRSPEED_SAFETY_FACTOR: f64 = 1.5;
const SPECIFIC_GRAVITY: f64 = 9.81;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aircraft {
    #[wasm_bindgen(skip)]
    pub id: String,

    #[wasm_bindgen(skip)]
    pub name: String,

    #[wasm_bindgen(skip)]
    #[serde(rename = "mtow")]
    pub raw_mtow: usize,

    pub takeoff: TakeoffPerformance,
    pub climb: ClimbPerformance,
    pub glide: GlidePerformance,
    pub landing: LandingPerformance,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct TakeoffPerformance {
    // Distance required to reach rotation speed (ft)
    ground_roll: usize,
    // Total distance required to clear a 50ft obstacle (ft)
    total_distance: usize,
    // Speed reached when clearing the 50ft obstacle (KIAS)
    speed: usize,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ClimbPerformance {
    // Speed for best Rate-of-Climb (KIAS)
    speed: usize,
    // Climb rate with full throttle and pitched for Vy (ft/min)
    rate: usize,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct GlidePerformance {
    // Ground distance covered per 1.000ft of altitude lost (nm)
    distance: f64,
    // Speed for best glide distance (KIAS)
    speed: usize,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LandingPerformance {
    // Ground roll required after touchdown to come to a complete stop (ft)
    ground_roll: usize,
    // Total distance required to clear a 50ft obstacle and come to a full stop (ft)
    total_distance: usize,
    // Speed when passing the 50ft obstacle (KIAS)
    speed: usize,
    /// Fastest descent speed in dirty configuration while maintaining landing speed (ft/min)
    descent_rate: usize,
}

#[wasm_bindgen]
impl Aircraft {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsValue {
        JsValue::from_str(&self.id)
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsValue {
        JsValue::from_str(&self.name)
    }

    /// Maximum takeoff weight (in kg)
    #[wasm_bindgen(getter)]
    pub fn mtow(&self) -> f64 {
        Mass::new::<pound>(self.raw_mtow as f64).get::<kilogram>()
    }
}

#[wasm_bindgen]
impl TakeoffPerformance {
    #[wasm_bindgen(getter, js_name = groundRoll)]
    pub fn ground_roll(&self) -> f64 {
        Length::new::<foot>(self.ground_roll as f64).get::<meter>()
    }

    #[wasm_bindgen(getter, js_name = totalDistance)]
    pub fn total_distance(&self) -> f64 {
        Length::new::<foot>(self.total_distance as f64).get::<meter>()
    }
}

#[wasm_bindgen]
impl ClimbPerformance {
    /// Maximum rate of climb (ft/min)
    #[wasm_bindgen(getter)]
    pub fn rate(&self) -> f64 {
        self.rate as f64
    }

    // Factor which when multiplied by the height gained yields the ground track covered
    #[wasm_bindgen(getter)]
    pub fn ratio(&self) -> f64 {
        // To prevent headaches, convert everything to metric units :D
        let speed = Velocity::new::<knot>(self.speed as f64).get::<meter_per_second>();
        let rate = Velocity::new::<foot_per_minute>(self.rate as f64).get::<meter_per_second>();

        // Rate equals the number of meters climbed per second, thus in one second we climb `rate` meters
        // Speed gives us the ground distance covered in per second, so in one second we cover `speed` meters on the diagonal
        // To get the ground track covered, we have to apply pythagoras on the triangle between height gained and diagonal distance covered.
        // a = speed * 1sec = speed
        // b = rate * 1sec = rate
        // c = ground track covered for each rate meters of height gained
        let track = (speed.powi(2) + rate.powi(2)).sqrt();

        // To normalise the slope and get a factor which when multiplied by the height gained yields the ground distance covered,
        // we divide the track by the height gained in one second (aka rate)
        track / rate
    }
}

impl GlidePerformance {
    pub fn speed(&self) -> Velocity {
        Velocity::new::<knot>(self.speed as f64)
    }

    /// Calculates units of height lost per units of ground track covered
    pub fn height_lost_for_ground_track(&self, distance: f64) -> f64 {
        distance / self.ratio()
    }

    /// Calculates meters of height lost per radians of turn commenced at the given bank (in radians)
    pub fn height_lost_in_turn(&self, angle: f64, bank: f64) -> f64 {
        let radius = self.turn_radius(bank);
        let distance = (radius * angle).abs();

        // TODO During a turn the airspeed has to be increased above the ideal gliding speed.
        //      Additionally, due to the bank some lift component is "wasted" on turning and thus overall vertical lift decreases further.
        //      Those factors will likely yield a worse number than we account for!
        // For now we will just use a safety factor roughly based on the load factor to compensate! But this is likely absolutely wrong ...
        let safety_factor = 1.0 / bank.cos();

        self.height_lost_for_ground_track(distance) * safety_factor
    }

    pub fn height_loss_over_geometric_path(&self, path: &DubinPath, bank: f64) -> f64 {
        match path {
            DubinPath::CSC(arc1, straight, arc2) => {
                self.height_lost_in_turn(arc1.angle().get::<radian>(), bank)
                    + self.height_lost_for_ground_track(straight.length().get::<meter>())
                    + self.height_lost_in_turn(arc2.angle().get::<radian>(), bank)
            }
            DubinPath::CCC(arc1, arc2, arc3) => {
                self.height_lost_in_turn(arc1.angle().get::<radian>(), bank)
                    + self.height_lost_in_turn(arc2.angle().get::<radian>(), bank)
                    + self.height_lost_in_turn(arc3.angle().get::<radian>(), bank)
            }
        }
    }

    pub fn height_loss_over_geographic_path(&self, path: &GeographicDubinPath, bank: f64) -> f64 {
        match path {
            GeographicDubinPath::CSC(arc1, straight, arc2) => {
                self.height_lost_in_turn(arc1.geometric_arc.angle().get::<radian>(), bank)
                    + self.height_lost_for_ground_track(
                        straight.geometric_tangent.length().get::<meter>(),
                    )
                    + self.height_lost_in_turn(arc2.geometric_arc.angle().get::<radian>(), bank)
            }
            GeographicDubinPath::CCC(arc1, arc2, arc3) => {
                self.height_lost_in_turn(arc1.geometric_arc.angle().get::<radian>(), bank)
                    + self.height_lost_in_turn(arc2.geometric_arc.angle().get::<radian>(), bank)
                    + self.height_lost_in_turn(arc3.geometric_arc.angle().get::<radian>(), bank)
            }
        }
    }
}

#[wasm_bindgen]
impl GlidePerformance {
    /// Units of distance covered per unit of height lost.
    /// Commonly expressed as a ratio e.g. `1:10` where `10` is the value returned.
    #[wasm_bindgen(getter)]
    pub fn ratio(&self) -> f64 {
        let height = Length::new::<foot>(1000.0);
        let track = Length::new::<nautical_mile>(self.distance);
        track.get::<meter>() / height.get::<meter>()
    }

    /// Turn radius in meters while gliding at the given bank (in radians)
    #[wasm_bindgen(js_name = turnRadius)]
    pub fn turn_radius(&self, bank: f64) -> f64 {
        // TODO This should really be based on the clean stall speed and how close we are to it!
        let speed = self.speed() * TURN_AIRSPEED_SAFETY_FACTOR;
        let gravity = Acceleration::new::<meter_per_second_squared>(SPECIFIC_GRAVITY);
        let radius = speed.powi(uom::typenum::P2::new()) / (gravity * bank.tan());
        radius.get::<meter>()
    }
}

#[wasm_bindgen]
impl LandingPerformance {
    #[wasm_bindgen(getter, js_name = groundRoll)]
    pub fn ground_roll(&self) -> f64 {
        Length::new::<foot>(self.ground_roll as f64).get::<meter>()
    }

    #[wasm_bindgen(getter, js_name = totalDistance)]
    pub fn total_distance(&self) -> f64 {
        Length::new::<foot>(self.total_distance as f64).get::<meter>()
    }

    // Factor which when multiplied by the height lost yields the ground track covered
    #[wasm_bindgen(getter)]
    pub fn descend_ratio(&self) -> f64 {
        // To prevent headaches, convert everything to metric units :D
        let speed = Velocity::new::<knot>(self.speed as f64).get::<meter_per_second>();
        let rate = Velocity::new::<foot_per_minute>(self.descent_rate as f64).get::<meter_per_second>();

        // Rate equals the number of meters descended per second, thus in one second we climb `rate` meters
        // Speed gives us the ground distance covered in per second, so in one second we cover `speed` meters on the diagonal
        // To get the ground track covered, we have to apply pythagoras on the triangle between height lost and diagonal distance covered.
        // a = speed * 1sec = speed
        // b = rate * 1sec = rate
        // c = ground track covered for each rate meters of height lost
        let track = (speed.powi(2) + rate.powi(2)).sqrt();

        // To normalise the slope and get a factor which when multiplied by the height lost yields the ground distance covered,
        // we divide the track by the height lost in one second (aka rate)
        track / rate
    }
}

impl LandingPerformance {
    pub fn ground_roll_on_surface(&self, surface: &SurfaceType) -> f64 {
        // TODO When the surface is wet, these numbers no longer apply
        match surface {
            SurfaceType::Asphalt => self.ground_roll(),
            SurfaceType::Gras => self.ground_roll() * 1.20,
            // TODO This is an unknown figure
            SurfaceType::Water => self.ground_roll(),
            _ => unreachable!(),
        }
    }

    pub fn total_distance_on_surface(&self, surface: &SurfaceType) -> f64 {
        let clearance_distance = self.total_distance() - self.ground_roll();
        let ground_roll = self.ground_roll_on_surface(&surface);

        clearance_distance + ground_roll
    }
}
