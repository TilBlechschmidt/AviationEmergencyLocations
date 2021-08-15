//! All performance numbers are at MTOW unless otherwise specified.
//! Additionally, values are given in a standard atmosphere,
//! with standard temperature, at MSL, and not account for wind.
//! Takeoff and landing figures are both with short-field technique.

use serde::{Deserialize, Serialize};
use uom::{
    si::{
        acceleration::meter_per_second_squared,
        f64::*,
        length::{foot, meter, nautical_mile},
        mass::pound,
        velocity::{foot_per_minute, knot, meter_per_second},
    },
    typenum::P2,
};
use uuid::Uuid;

use super::SurfaceType;

// Factor by which airspeed will be increased by in turns to avoid stalls
const TURN_AIRSPEED_SAFETY_FACTOR: f64 = 1.5;
const SPECIFIC_GRAVITY: f64 = 9.81;

pub type AircraftIdentifier = String;

fn turn_radius(speed: Velocity, bank: Angle) -> Length {
    let speed = speed * TURN_AIRSPEED_SAFETY_FACTOR;
    let gravity = Acceleration::new::<meter_per_second_squared>(SPECIFIC_GRAVITY);
    speed.powi(P2::new()) / (gravity * bank.tan())
}

fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aircraft {
    #[serde(default = "new_uuid")]
    pub id: AircraftIdentifier,
    pub name: String,

    // Maximum takeoff weight (pounds)
    mtow: usize,

    pub takeoff: TakeoffPerformance,
    pub climb: ClimbPerformance,
    pub glide: GlidePerformance,
    pub landing: LandingPerformance,
}

impl Aircraft {
    pub fn maximum_takeoff_weight(&self) -> Mass {
        Mass::new::<pound>(self.mtow as f64)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeoffPerformance {
    // Distance required to reach rotation speed (ft)
    ground_roll: usize,
    // Total distance required to clear a 50ft obstacle (ft)
    total_distance: usize,
    // Speed reached when clearing the 50ft obstacle (KIAS)
    speed: usize,
}

impl TakeoffPerformance {
    pub fn ground_roll(&self) -> Length {
        Length::new::<foot>(self.ground_roll as f64)
    }

    pub fn total_distance(&self) -> Length {
        Length::new::<foot>(self.total_distance as f64)
    }

    pub fn speed(&self) -> Velocity {
        Velocity::new::<knot>(self.speed as f64)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClimbPerformance {
    // Speed for best Rate-of-Climb (KIAS)
    speed: usize,
    // Climb rate with full throttle and pitched for Vy (ft/min)
    rate: usize,
}

impl ClimbPerformance {
    pub fn speed(&self) -> Velocity {
        Velocity::new::<knot>(self.speed as f64)
    }

    pub fn rate(&self) -> Velocity {
        Velocity::new::<foot_per_minute>(self.rate as f64)
    }

    // Factor which when multiplied by the height gained yields the ground track covered
    fn factor(&self) -> f64 {
        // To prevent headaches, convert everything to metric units :D
        let speed = self.speed().get::<meter_per_second>();
        let rate = self.rate().get::<meter_per_second>();

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

    pub fn ground_track_for_height_gained(&self, height: Length) -> Length {
        let distance_covered_meters = height.get::<meter>() * self.factor();

        Length::new::<meter>(distance_covered_meters)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlidePerformance {
    // Ground distance covered per 1.000ft of altitude lost (nm)
    distance: f64,
    // Speed for best glide distance (KIAS)
    speed: usize,
}

impl GlidePerformance {
    fn distance_per_thousand_feet(&self) -> Length {
        Length::new::<nautical_mile>(self.distance)
    }

    pub fn speed(&self) -> Velocity {
        Velocity::new::<knot>(self.speed as f64)
    }

    // Factor which when multiplied by the height lost in feet yields the ground track covered in meters
    fn factor(&self) -> f64 {
        // Dividing the ground distance covered in a thousand feet of height by a thousand feet
        // yields a value which when multiplied by any unit gives the corresponding ground track.
        self.distance_per_thousand_feet().get::<meter>() / 1000.0
    }

    pub fn turn_radius(&self, bank: Angle) -> Length {
        turn_radius(self.speed(), bank)
    }

    pub fn ground_track_for_height_lost(&self, height: Length) -> Length {
        let distance_covered_meters = height.get::<foot>() * self.factor();

        Length::new::<meter>(distance_covered_meters)
    }

    pub fn height_lost_for_ground_track(&self, distance: Length) -> Length {
        let height_lost_feet = distance.get::<meter>() / self.factor();

        Length::new::<foot>(height_lost_feet)
    }

    pub fn height_lost_for_turn(&self, bank: Angle, angle: Angle) -> Length {
        let radius = self.turn_radius(bank);
        let distance = (radius * angle).abs();

        // TODO During a turn the airspeed has to be increased above the ideal gliding speed.
        //      Additionally, due to the bank some lift component is "wasted" on turning and thus overall vertical lift decreases further.
        //      Those factors will likely yield a worse number than we account for!
        // For now we will just use a safety factor roughly based on the load factor to compensate! But this is likely absolutely wrong ...
        let safety_factor = 1.0 / bank.cos();

        self.height_lost_for_ground_track(distance) * safety_factor
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LandingPerformance {
    // Ground roll required after touchdown to come to a complete stop (ft)
    ground_roll: usize,
    // Total distance required to clear a 50ft obstacle and come to a full stop (ft)
    total_distance: usize,
    // Speed when passing the 50ft obstacle (KIAS)
    speed: usize,
}

impl LandingPerformance {
    pub fn speed(&self) -> Velocity {
        Velocity::new::<knot>(self.speed as f64)
    }

    pub fn ground_roll(&self) -> Length {
        Length::new::<foot>(self.ground_roll as f64)
    }

    pub fn total_distance(&self) -> Length {
        Length::new::<foot>(self.total_distance as f64)
    }

    pub fn ground_roll_on_surface(&self, surface: &SurfaceType) -> Length {
        match surface {
            SurfaceType::Asphalt => self.ground_roll(),
            SurfaceType::Gras => self.ground_roll() * 1.45,
            // TODO This is an unknown figure
            SurfaceType::Water => self.ground_roll(),
        }
    }

    pub fn total_distance_on_surface(&self, surface: &SurfaceType) -> Length {
        let clearance_distance = self.total_distance() - self.ground_roll();
        let ground_roll = self.ground_roll_on_surface(&surface);

        clearance_distance + ground_roll
    }
}
