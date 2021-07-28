//! All performance numbers are at MTOW unless otherwise specified.
//! Additionally, values are given in a standard atmosphere,
//! with standard temperature, at MSL, and not account for wind.
use uom::si::{
    f64::*,
    length::{foot, meter, nautical_mile},
    mass::pound,
    velocity::{foot_per_minute, knot, meter_per_second},
};

pub struct Aircraft {
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

    pub fn ground_track_for_height_lost(&self, height: Length) -> Length {
        let distance_covered_meters = height.get::<foot>() * self.factor();

        Length::new::<meter>(distance_covered_meters)
    }

    pub fn height_lost_for_ground_track(&self, distance: Length) -> Length {
        let height_lost_feet = distance.get::<meter>() / self.factor();

        Length::new::<foot>(height_lost_feet)
    }
}

pub struct LandingPerformance {
    // Ground roll required after touchdown to come to a complete stop (ft)
    ground_roll: usize,
    // Total distance required to clear a 50ft obstacle and come to a full stop (ft)
    total_distance: usize,
    // Speed for touchdown
    speed: usize,
}

impl LandingPerformance {
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
