use geo::prelude::EuclideanDistance;

use super::structs::*;

fn ccc_path(
    start: &DirectedPoint,
    end: &DirectedPoint,
    start_circle: &Circle,
    end_circle: &Circle,
    radius: f64,
) -> Option<DubinPath> {
    let circle_distance = start_circle.center.euclidean_distance(&end_circle.center);

    // CCC paths only make sense when the distance between the two points is less than 4r
    if start_circle.center.euclidean_distance(&end_circle.center) >= radius * 4.0 {
        return None;
    }

    // Calculate angle by which the vector between the two circles has to be rotated
    let mut theta = (circle_distance / (4.0 * radius)).acos();
    let delta = (end_circle.center.y() - start_circle.center.y())
        .atan2(end_circle.center.x() - start_circle.center.x());
    match start_circle.direction {
        Direction::Left => theta = delta - theta,
        Direction::Right => theta += delta,
    }

    // Derive the third circle
    let center = Point::new(
        start_circle.center.x() + 2.0 * radius * theta.cos(),
        start_circle.center.y() + 2.0 * radius * theta.sin(),
    );
    let circle = Circle {
        center,
        radius,
        direction: start_circle.direction.opposite(),
    };

    // Find the two tangent points
    let first_crossover = (center + start_circle.center) / 2.0;
    let second_crossover = (center + end_circle.center) / 2.0;

    // Build the data structures
    let arc1 = Arc::new(**start, first_crossover, *start_circle);
    let arc2 = Arc::new(first_crossover, second_crossover, circle);
    let arc3 = Arc::new(second_crossover, **end, *end_circle);

    Some(DubinPath::CCC(arc1, arc2, arc3))
}

pub(super) fn ccc_paths(
    start: &DirectedPoint,
    end: &DirectedPoint,
    circles: &CircleSet,
    radius: f64,
) -> Vec<DubinPath> {
    // Build a list of possible combinations
    let combinations = [
        (&circles.start_left, &circles.end_left),
        (&circles.start_right, &circles.end_right),
    ];

    // Calculate the path for all
    combinations
        .iter()
        .flat_map(|(start_circle, end_circle)| {
            ccc_path(&start, &end, start_circle, end_circle, radius)
        })
        .collect()
}
