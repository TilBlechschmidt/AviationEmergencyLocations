use super::structs::*;

fn csc_path(
    start: &DirectedPoint,
    end: &DirectedPoint,
    start_circle: &Circle,
    end_circle: &Circle,
) -> Option<DubinPath> {
    // Calculate the tangent
    if let Some(tangent) = start_circle.tangent_to(&end_circle) {
        // Calculate the angles to travel
        let departure_arc = start_circle.arc(**start, tangent.start);
        let arrival_arc = end_circle.arc(tangent.end, **end);

        Some(DubinPath::CSC(departure_arc, tangent, arrival_arc))
    } else {
        None
    }
}

pub(super) fn csc_paths(
    start: &DirectedPoint,
    end: &DirectedPoint,
    circles: &CircleSet,
) -> Vec<DubinPath> {
    // Build a list of possible combinations
    let combinations = [
        (&circles.start_left, &circles.end_left),
        (&circles.start_left, &circles.end_right),
        (&circles.start_right, &circles.end_left),
        (&circles.start_right, &circles.end_right),
    ];

    // Calculate the path for all
    combinations
        .iter()
        .flat_map(|(start_circle, end_circle)| csc_path(&start, &end, start_circle, end_circle))
        .collect()
}
