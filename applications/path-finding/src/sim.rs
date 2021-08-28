use super::data::LocationIdentifier;
use super::dubin::{DubinPath, GeographicArc, GeographicDubinPath};
use crate::{
    data::{Aircraft, Location, Output},
    dubin::{
        calculate_dubin_path_candidates, calculate_georeferenced_dubin_path_candidates,
        geometric_point_to_geographic, relative_geometric_position_of, Direction,
    },
};
use geo::{
    convex_hull::quick_hull,
    coords_iter::CoordsIter,
    geodesic_distance::GeodesicDistance,
    point,
    prelude::{Bearing, HaversineDestination, HaversineDistance},
    simplify::Simplify,
    Coordinate, LineString, Point, Polygon,
};
use geojson::{Feature, FeatureCollection, GeoJson, Value};
use serde_json::{to_value, Map};
use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::TryInto,
    f32::consts::FRAC_2_PI,
    f64::consts::{FRAC_PI_2, TAU},
    fs::File,
    io::Write,
};
use uom::si::{
    angle::{degree, radian},
    f64::{Angle, Length},
    length::{foot, meter},
};

fn path_to_geojson(path: &GeographicDubinPath, location: &str) -> Feature {
    let points = match path {
        GeographicDubinPath::CSC(arc1, tangent, arc2) => {
            let mut points = Vec::new();
            points.append(&mut arc1.points());
            points.push(tangent.start);
            points.push(tangent.end);
            points.append(&mut arc2.points());
            points
        }
        GeographicDubinPath::CCC(arc1, arc2, arc3) => {
            let mut points = Vec::new();
            points.append(&mut arc1.points());
            points.append(&mut arc2.points());
            points.append(&mut arc3.points());
            points
        }
    };

    let line: LineString<f64> = points.into();
    let mut properties = Map::new();
    properties.insert(String::from("location"), to_value(location).unwrap());

    Feature {
        bbox: None,
        geometry: Some((&line).into()),
        id: None,
        properties: Some(properties),
        foreign_members: None,
    }
}

pub struct LandingOption {
    location_id: LocationIdentifier,
    path: GeographicDubinPath,
    height_loss: Length,
}

pub struct Simulation {
    pub bank: Angle,
    pub resolution: Length,
    pub altitudes: Vec<Length>,
}

impl Simulation {
    fn height_loss_over_geometric_path(&self, aircraft: &Aircraft, path: &DubinPath) -> Length {
        match path {
            DubinPath::CSC(arc1, straight, arc2) => {
                aircraft.glide.height_lost_for_turn(self.bank, arc1.angle())
                    + aircraft
                        .glide
                        .height_lost_for_ground_track(straight.length())
                    + aircraft.glide.height_lost_for_turn(self.bank, arc2.angle())
            }
            DubinPath::CCC(arc1, arc2, arc3) => {
                aircraft.glide.height_lost_for_turn(self.bank, arc1.angle())
                    + aircraft.glide.height_lost_for_turn(self.bank, arc2.angle())
                    + aircraft.glide.height_lost_for_turn(self.bank, arc3.angle())
            }
        }
    }

    fn height_loss_over_path(&self, aircraft: &Aircraft, path: &GeographicDubinPath) -> Length {
        match path {
            GeographicDubinPath::CSC(arc1, straight, arc2) => {
                aircraft.glide.height_lost_for_turn(self.bank, arc1.theta)
                    + aircraft
                        .glide
                        .height_lost_for_ground_track(straight.length())
                    + aircraft.glide.height_lost_for_turn(self.bank, arc2.theta)
            }
            GeographicDubinPath::CCC(arc1, arc2, arc3) => {
                aircraft.glide.height_lost_for_turn(self.bank, arc1.theta)
                    + aircraft.glide.height_lost_for_turn(self.bank, arc2.theta)
                    + aircraft.glide.height_lost_for_turn(self.bank, arc3.theta)
            }
        }
    }

    fn compare_geometric_path_height_loss(
        &self,
        aircraft: &Aircraft,
        x: &DubinPath,
        y: &DubinPath,
    ) -> Ordering {
        let x_height = self.height_loss_over_geometric_path(&aircraft, &x);
        let y_height = self.height_loss_over_geometric_path(&aircraft, &y);

        x_height.partial_cmp(&y_height).unwrap_or(Ordering::Equal)
    }

    fn compare_path_height_loss(
        &self,
        aircraft: &Aircraft,
        x: &GeographicDubinPath,
        y: &GeographicDubinPath,
    ) -> Ordering {
        let x_height = self.height_loss_over_path(&aircraft, &x);
        let y_height = self.height_loss_over_path(&aircraft, &y);

        x_height.partial_cmp(&y_height).unwrap_or(Ordering::Equal)
    }

    fn landing_options(
        &self,
        aircraft: &Aircraft,
        locations: &Vec<Location>,
        position: Point<f64>,
        heading: Angle,
    ) -> Vec<LandingOption> {
        locations
            .iter()
            .flat_map(|location| {
                location
                    .landable_points(&aircraft, self.resolution)
                    .iter()
                    .flat_map(|(point, direction)| {
                        calculate_georeferenced_dubin_path_candidates(
                            position,
                            *point,
                            heading,
                            *direction,
                            aircraft.glide.turn_radius(self.bank),
                        )
                        .into_iter()
                        .min_by(|x, y| self.compare_path_height_loss(&aircraft, x, y))
                    })
                    .min_by(|x, y| self.compare_path_height_loss(&aircraft, x, y))
                    .map(|path| LandingOption {
                        location_id: location.id.clone(),
                        height_loss: self.height_loss_over_path(&aircraft, &path),
                        path,
                    })
            })
            .collect::<Vec<_>>()
    }

    fn range_profile(
        &self,
        aircraft: &Aircraft,
        location: &Location,
        altitude: Length,
        epsilon: f64,
    ) -> Polygon<f64> {
        let resolution = self.resolution;
        let maximum_range = aircraft.glide.ground_track_for_height_lost(altitude) * 2.0;
        let meter_per_pixel = resolution.get::<meter>().ceil() as usize;
        let texture_size = maximum_range.get::<meter>().ceil() as usize / meter_per_pixel;
        let offset = maximum_range.get::<meter>().ceil() / 2.0;
        let radius = aircraft.glide.turn_radius(self.bank);

        let mut reachable_points = Vec::with_capacity(texture_size * texture_size);
        let mut reverse_reachable_points = Vec::with_capacity(texture_size * texture_size);

        // 1. For each direction (forward/reverse) figure out, with the given precision, all points that can reach either the start or the end
        for py in 0..texture_size {
            for px in 0..texture_size {
                let x = (px * meter_per_pixel) as f64 - offset;
                let y = (py * meter_per_pixel) as f64 - offset;
                let aircraft_position = Point::new(x, y);

                let target_angle_in_reach = |geographic_target_bearing: Angle| -> bool {
                    let geometric_target_angle =
                        geographic_target_bearing - Angle::new::<radian>(FRAC_PI_2);
                    let unreachable_angle = (0..360).into_iter().find(|alpha| {
                        let aircraft_heading = Angle::new::<degree>(*alpha as f64);

                        let shortest_path = calculate_dubin_path_candidates(
                            aircraft_position,
                            Point::new(0.0, 0.0),
                            aircraft_heading,
                            geometric_target_angle,
                            radius,
                        )
                        .into_iter()
                        .min_by(|x, y| self.compare_geometric_path_height_loss(&aircraft, x, y));

                        shortest_path
                            .map(|path| {
                                self.height_loss_over_geometric_path(&aircraft, &path) > altitude
                            })
                            .unwrap_or(true)
                    });

                    unreachable_angle.is_none()
                };

                if target_angle_in_reach(location.bearing()) {
                    reachable_points.push(Point::new(x, y));
                }

                if location.reversible && target_angle_in_reach(location.reverse_bearing()) {
                    reverse_reachable_points.push(Point::new(x, y));
                }
            }
        }

        // 2. Shift all the previously calculated points by the landable locations and merge both directions into one vector
        let landable_points = location.landable_points(&aircraft, resolution);
        let centroid = location.centroid();
        let mut all_reachable_points = Vec::new();

        for (landable_point, landable_bearing) in landable_points.into_iter() {
            let offset_from_centroid = relative_geometric_position_of(landable_point, centroid);

            let points = if landable_bearing == location.bearing() {
                &reachable_points
            } else if location.reversible && landable_bearing == location.reverse_bearing() {
                &reverse_reachable_points
            } else {
                unreachable!()
            };

            // Move all points from their respective relative position to `start` or `end` to a position relative to `landable_point`
            let mut shifted_points = points
                .iter()
                .map(|point| (*point + offset_from_centroid).0)
                .collect();
            all_reachable_points.append(&mut shifted_points);
        }

        // Simplify the point set by calculating the convex hull
        let convex_hull = quick_hull(&mut all_reachable_points);
        // TODO Make sure all pixels within the hull are filled

        // Reduce the complexity of the polygon to save some space
        let simplified_hull = convex_hull.simplify(&epsilon);

        // Convert the geometric convex hull to geographic coordinates
        let geographic_convex_hull: LineString<f64> = simplified_hull
            .into_points()
            .into_iter()
            .map(|point| geometric_point_to_geographic(point, centroid))
            .collect::<Vec<_>>()
            .into();

        println!(
            "Calculated range profile for {} - {} - {}",
            location.name,
            aircraft.name,
            altitude.get::<foot>()
        );

        Polygon::new(geographic_convex_hull, vec![])
    }

    pub fn generate_range_profiles(
        &self,
        location: &Location,
        aircraft: &Aircraft,
        epsilon: f64,
    ) -> HashMap<usize, GeoJson> {
        self.altitudes
            .iter()
            .map(|altitude| {
                let polygon = self.range_profile(aircraft, location, *altitude, epsilon);
                let geojson = GeoJson::Feature(Feature {
                    bbox: None,
                    geometry: Some((&polygon).into()),
                    id: None,
                    properties: None,
                    foreign_members: None,
                });

                (altitude.get::<foot>().round() as usize, geojson)
            })
            .collect()
    }

    fn generate_aircraft_distance_profile(
        &self,
        aircraft: &Aircraft,
        altitude: Length,
    ) -> Vec<(f64, f64)> {
        let radius = aircraft.glide.turn_radius(self.bank);
        let maximum_range = aircraft
            .glide
            .ground_track_for_height_lost(altitude)
            .get::<meter>() as i32
            * 2;
        let resolution = 0.1;
        let origin = Point::new(0.0, 0.0);
        let origin_angle = Angle::new::<degree>(0.0);

        let find_edge_distance = |center: Point<f64>, ray_angle: f64| {
            let is_distance_in_reach = |distance: f64| {
                let location = Point::new(
                    center.y() + distance * ray_angle.cos(),
                    center.x() + distance * ray_angle.sin(),
                );

                let unreachable_angle = (0..360).into_iter().find(|alpha| {
                    let aircraft_heading = Angle::new::<degree>(*alpha as f64);

                    let shortest_path = calculate_dubin_path_candidates(
                        location,
                        origin,
                        aircraft_heading,
                        origin_angle,
                        radius,
                    )
                    .into_iter()
                    .min_by(|x, y| self.compare_geometric_path_height_loss(&aircraft, x, y));

                    shortest_path
                        .map(|path| {
                            self.height_loss_over_geometric_path(&aircraft, &path) > altitude
                        })
                        .unwrap_or(true)
                });

                if let Some(angle) = unreachable_angle {
                    println!("R{} U{}", ray_angle.to_degrees().round(), angle);
                }

                return unreachable_angle.is_none();
            };

            // Run a binary search over the distance range [0..maximum_range] to find the edge
            let mut low = 0.0;
            let mut high = maximum_range as f64;

            while high - low >= resolution {
                let middle = (high + low) / 2.0;
                let in_reach = is_distance_in_reach(middle);

                if in_reach {
                    low = middle;
                } else {
                    high = middle;
                }
            }

            return (high + low) / 2.0;
        };

        // We calculate the range rays from an offset. The furthest expansion of the range on the X-axis is not on the same Y-level as the origin.
        // Instead, it is located further south by `radius` because assuming the plane flies straight towards the runway, it still needs to make a turn at the end.
        // So the "shortest" path will always be "head straight west towards the point which is `radius` south of the approach end and then make a 90º turn in".
        //
        // Calculating from this offset location allows us to "cut" the ray array into two halfes. The front half (90º-270º) and back half (270º-90º).
        // When the runway is longer or approachable from both ends, we can stitch the pieces together by connecting them with straight lines.
        // This method works for altitudes above which the range shape is a solid polygon without any "kinks" in it. Usually at or above 1.500ft
        //
        // Additionally, we offset the angle by 90º so that it is easier to use range expressions on the array (namely [0; 18] and [18; 36]).
        let offset_y = radius.get::<meter>();
        let offset_origin = Point::new(0.0, -offset_y);

        (0..36)
            .map(|raw_angle| {
                let ray_angle = ((raw_angle * 10 + 90) as f64).to_radians();
                let distance = find_edge_distance(offset_origin, ray_angle);
                (ray_angle, distance)
            })
            .collect()
    }

    pub fn generate_aircraft_distance_regression(
        &self,
        aircraft: &Aircraft,
    ) -> Vec<(f64, f64, f64)> {
        let altitude_a = 1500.0;
        let altitude_b = 2500.0;
        let altitude_delta = altitude_b - altitude_a;

        let sample_a = self
            .generate_aircraft_distance_profile(aircraft, Length::new::<foot>(altitude_a))
            .into_iter();

        let sample_b = self
            .generate_aircraft_distance_profile(aircraft, Length::new::<foot>(altitude_b))
            .into_iter();

        let distance_regression = sample_a
            .zip(sample_b)
            .map(|((angle_a, distance_a), (angle_b, distance_b))| {
                assert_eq!(angle_a, angle_b);

                // Calculating the slope for the line equation of the form `m(x - x0) + b = y`
                let distance_delta = distance_b - distance_a;
                let slope = distance_delta / altitude_delta;

                // Converting `m(x - x0) + b` into `m * x - (m * x0) + b` where `b` is `distance_a`
                let offset = -(slope * altitude_a) + distance_a;

                // Output line as (α, m, b) with the following equation: `mx + b = y`
                (angle_a, slope, offset)
            })
            .collect::<Vec<_>>();

        // Code below is to verify the maximum error introduced through the regression.
        // All regressions are estimating smaller ranges up to 30m (~10m in the front half).
        // Overall, a <2% error especially on the side of caution is acceptable for the usability and performance improvements gained :)
        //
        // let mut error = 0.0;
        // let mut msg = "".into();
        // for i in 15..25 {
        //     let altitude = i as f64 * 100.0;
        //     let profile =
        //         self.generate_aircraft_distance_profile(&aircraft, Length::new::<foot>(altitude));

        //     profile
        //         .into_iter()
        //         .enumerate()
        //         .for_each(|(index, (angle, distance))| {
        //             let (regression_angle, slope, offset) = distance_regression.get(index).unwrap();
        //             assert_eq!(*regression_angle, angle);

        //             let regression_distance = slope * altitude + offset;
        //             let e = (regression_distance - distance).abs();
        //             if e > error {
        //                 error = e;
        //                 msg = format!(
        //                     "{} == {} @ {} - {}º",
        //                     regression_distance.round(),
        //                     distance.round(),
        //                     altitude,
        //                     angle.to_degrees().round()
        //                 );
        //             }
        //         });
        // }
        // println!("{} | {}", error, msg);

        distance_regression
    }
}
