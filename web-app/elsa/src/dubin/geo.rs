use super::*;
use ::geo::{
    geodesic_distance::GeodesicDistance,
    point,
    prelude::{
        Bearing, EuclideanDistance, GeodesicIntermediate, HaversineDestination, HaversineDistance,
    },
};
use std::f64::consts::{FRAC_PI_2, TAU};

#[derive(Debug)]
pub struct GeographicTangent {
    pub start: Point,
    pub end: Point,
}

impl GeographicTangent {
    pub fn length(&self) -> Length {
        Length::new::<meter>(self.start.geodesic_distance(&self.end))
    }
}

#[derive(Debug)]
pub struct GeographicArc {
    pub start: Point,
    pub end: Point,
    pub theta: Angle,

    pub center: Point,
    pub radius: Length,
    pub direction: Direction,

    geometric_arc: Arc,
    geographic_origin: Point,
}

impl GeographicArc {
    pub fn points(&self) -> Vec<Point> {
        self.geometric_arc
            .points()
            .into_iter()
            .map(|point| geometric_point_to_geographic(point, self.geographic_origin))
            .rev()
            .collect()
    }
}

#[derive(Debug)]
pub enum GeographicDubinPath {
    CSC(GeographicArc, GeographicTangent, GeographicArc),
    CCC(GeographicArc, GeographicArc, GeographicArc),
}

pub fn geometric_point_to_geographic(point: Point, geographic_origin: Point) -> Point {
    if point.x() == 0.0 && point.y() == 0.0 {
        return geographic_origin;
    }

    let geometric_origin = point!(x: 0.0, y: 0.0);
    let distance = point.euclidean_distance(&geometric_origin);
    let bearing = (point.y().atan2(point.x()) + FRAC_PI_2).to_degrees();

    geographic_origin.haversine_destination(bearing, distance)
}

pub fn relative_geometric_position_of(geographic_point: Point, to_geographic_target: Point) -> Point {
    // Convert the geographic bearing between `start` and `end` to a geometric angle.
    let bearing = to_geographic_target.bearing(geographic_point).to_radians() - FRAC_PI_2;
    let distance = to_geographic_target.haversine_distance(&geographic_point);

    if geographic_point == to_geographic_target {
        to_geographic_target
    } else {
        point!(x: bearing.cos() * distance, y: bearing.sin() * distance)
    }
}

fn tangent_to_geographic(tangent: &Tangent, geographic_origin: Point) -> GeographicTangent {
    GeographicTangent {
        start: geometric_point_to_geographic(tangent.start, geographic_origin),
        end: geometric_point_to_geographic(tangent.end, geographic_origin),
    }
}

fn arc_to_geographic(arc: &Arc, geographic_origin: Point) -> GeographicArc {
    let circle = arc.circle;
    let center = geometric_point_to_geographic(circle.center, geographic_origin);
    let start = geometric_point_to_geographic(arc.start, geographic_origin);
    let end = geometric_point_to_geographic(arc.end, geographic_origin);

    GeographicArc {
        start,
        end,
        theta: arc.angle(),

        center,
        radius: circle.radius(),
        direction: circle.direction,

        geometric_arc: arc.clone(),
        geographic_origin,
    }
}

fn geometric_dubin_path_to_geographic(
    path: &DubinPath,
    geographic_origin: Point,
) -> GeographicDubinPath {
    match path {
        DubinPath::CSC(arc1, straight, arc2) => GeographicDubinPath::CSC(
            arc_to_geographic(arc1, geographic_origin),
            tangent_to_geographic(straight, geographic_origin),
            arc_to_geographic(arc2, geographic_origin),
        ),
        DubinPath::CCC(arc1, arc2, arc3) => GeographicDubinPath::CCC(
            arc_to_geographic(arc1, geographic_origin),
            arc_to_geographic(arc2, geographic_origin),
            arc_to_geographic(arc3, geographic_origin),
        ),
    }
}

pub fn calculate_georeferenced_dubin_path_candidates(
    start: Point,
    end: Point,
    start_bearing: Angle,
    end_bearing: Angle,
    radius: Distance,
) -> Vec<GeographicDubinPath> {
    // Convert the bearings to geometric angles in the local coordinate system
    let start_angle = start_bearing - Angle::new::<radian>(FRAC_PI_2);
    let end_angle = end_bearing - Angle::new::<radian>(FRAC_PI_2);

    // Assert `end` as the geometric origin and convert start into a geometric point in this coordinate system
    let geometric_end = point!(x: 0.0, y: 0.0);
    let geometric_start = relative_geometric_position_of(start, end);

    // Calculate the dubin path candidates in our coordinate system
    let candidates = calculate_dubin_path_candidates(
        geometric_start,
        geometric_end,
        start_angle,
        end_angle,
        radius,
    );

    // Convert every DubinPath instance to use geographic coordinates instead of geometric ones
    let geographic_origin = end;
    candidates
        .into_iter()
        .map(|path| geometric_dubin_path_to_geographic(&path, geographic_origin))
        .collect()
}
