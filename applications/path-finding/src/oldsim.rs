use super::*;
use geo::Coordinate;
use std::{f64::consts::PI, sync::Arc};
use uom::{
    fmt::DisplayStyle,
    si::{
        acceleration::meter_per_second_squared,
        angle::{degree, radian},
        f64::*,
        length::foot,
        velocity::meter_per_second,
    },
    typenum::P2,
};

struct SimulationParameters {
    /// Angle at which turns will be executed
    bank_angle: Angle,

    /// Multiplier for the airspeed during turns to account for increased load-factor
    turn_airspeed_factor: f64,

    /// Height below which a safe landing is unlikely due to obstacle clearance
    minimum_height: Altitude,
    /// Percentage of the starting altitude that can be bled off using measures like S-turns, flaps, and side-slips
    maximum_height_factor: f64,

    /// Specific gravity, for flying RC planes on Mars or smth :D
    gravity: Acceleration,
}

impl SimulationParameters {
    pub fn new(
        bank_angle: Angle,
        turn_airspeed_factor: f64,
        minimum_height: Altitude,
        maximum_height_factor: f64,
    ) -> Self {
        Self {
            bank_angle,
            turn_airspeed_factor,

            minimum_height,
            maximum_height_factor,

            gravity: Acceleration::new::<meter_per_second_squared>(9.81),
        }
    }
}

struct GlideParameters {
    slope: Slope,
    speed: Velocity,
}

impl GlideParameters {
    pub fn new(slope: Slope, speed: Velocity) -> Self {
        Self { slope, speed }
    }
}

pub struct Simulation {
    param: SimulationParameters,
    glide: GlideParameters,
}

impl Simulation {
    fn new(param: SimulationParameters, glide: GlideParameters) -> Self {
        Self { param, glide }
    }

    // Calculates the turn radius in a fixed bank angle turn
    pub fn turn_radius(&self) -> Distance {
        let speed = self.glide.speed * self.param.turn_airspeed_factor;
        speed.powi(P2::new()) / (self.param.gravity * self.param.bank_angle.tan())
    }

    // Calculates the height loss for a turn by the given angle in radians
    pub fn altitude_lost_in_turn(&self, angle: Angle) -> Altitude {
        let fraction_of_full_turn = angle.get::<radian>().abs() / (2.0 * PI);
        let radius = self.turn_radius();
        let distance_covered = 2.0 * PI * radius * fraction_of_full_turn;
        let height_lost = distance_covered / self.glide.slope;

        // TODO As the airspeed is increased to `self.param.turn_airspeed_factor`, the glide slope worsens!
        //      Thus, the actual real-world value is probably *worse* than the calculated value.

        height_lost
    }

    // Calculates the height loss when covering a certain ground distance
    pub fn altitude_lost_over_distance(&self, distance: Distance) -> Altitude {
        distance / self.glide.slope
    }

    fn start(self, altitude: Altitude, location: Location) -> SimulationState {
        SimulationState {
            altitude,
            location,
            simulation: Arc::new(self),
        }
    }
}

#[derive(Clone)]
struct SimulationState {
    /// Remaining height AGL
    altitude: Altitude,

    /// Location relative to the runway start.
    /// X-axis is parallel to the runway, pointing down the run.
    /// Y-axis points perpendicular to the runway in the direction that is closest to north.
    location: Location,

    /// Reference to the underlying simulation
    simulation: Arc<Simulation>,
}

pub fn build_simulation() -> Simulation {
    let param = SimulationParameters::new(
        Angle::new::<degree>(45.0),
        1.5,
        Altitude::new::<foot>(50.0),
        0.5,
    );

    let glide = GlideParameters::new(9.18, Velocity::new::<meter_per_second>(31.38));

    let sim = Simulation { param, glide };

    // let lost = sim.altitude_lost_in_turn(240f64.to_radians());
    // println!(
    //     "{:?}",
    //     lost.into_format_args(foot, DisplayStyle::Description)
    // );

    sim
}
