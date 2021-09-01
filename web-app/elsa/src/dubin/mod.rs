mod ccc;
mod csc;
mod geo;
mod structs;

type Distance = Length;
type Location = (Distance, Distance);

use csc::csc_paths;
use structs::*;
use uom::si::{angle::radian, f64::*, length::meter};

pub use self::geo::*;
pub use structs::{Arc, Circle, Direction, DubinPath, Tangent};

use self::ccc::ccc_paths;

pub fn calculate_dubin_path_candidates(
    start: Point,
    end: Point,
    start_angle: Angle,
    end_angle: Angle,
    radius: Distance,
) -> Vec<DubinPath> {
    // Internally we use radians and meters (obviously), convert everything up-front :)
    let start = DirectedPoint {
        point: start,
        angle: start_angle.get::<radian>(),
    };

    let end = DirectedPoint {
        point: end,
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
