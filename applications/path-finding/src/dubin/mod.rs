mod ccc;
mod csc;
mod structs;

use crate::{Distance, Location};
use csc::csc_paths;
use structs::*;
use uom::si::{angle::radian, f64::*, length::meter};

pub use structs::{Arc, Circle, Direction, DubinPath, Tangent};

use self::ccc::ccc_paths;

pub fn calculate_dubin_path_candidates(
    start: Location,
    end: Location,
    start_angle: Angle,
    end_angle: Angle,
    radius: Distance,
) -> Vec<DubinPath> {
    // Internally we use radians and meters (obviously), convert everything up-front :)
    let start = DirectedPoint {
        point: Point::new(start.0.get::<meter>(), start.1.get::<meter>()),
        angle: start_angle.get::<radian>(),
    };

    let end = DirectedPoint {
        point: Point::new(end.0.get::<meter>(), end.1.get::<meter>()),
        angle: end_angle.get::<radian>(),
    };

    let radius = radius.get::<meter>();

    calculate_paths(&start, &end, radius)
}

fn calculate_paths(start: &DirectedPoint, end: &DirectedPoint, radius: f64) -> Vec<DubinPath> {
    // Build the four circles
    let circles = CircleSet::new(&start, &end, radius);

    // Calculate the shortest CSC and CCC paths
    let mut paths = Vec::with_capacity(4);

    paths.append(&mut csc_paths(&start, &end, &circles));
    paths.append(&mut ccc_paths(&start, &end, &circles, radius));

    paths
}
