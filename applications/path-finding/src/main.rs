#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{
    data::{Aircraft, Location, Output},
    dubin::{
        calculate_dubin_path_candidates, calculate_georeferenced_dubin_path_candidates,
        geometric_point_to_geographic, relative_geometric_position_of, Direction,
    },
};
use data::LocationIdentifier;
use dubin::{DubinPath, GeographicArc, GeographicDubinPath};
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
use image::{GrayImage, Luma, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use rayon::prelude::*;
use serde_json::{to_value, Map};
use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::TryInto,
    f64::consts::{FRAC_PI_2, TAU},
    fs::File,
    io::Write,
};
use uom::si::{
    angle::{degree, radian},
    f64::{Angle, Length},
    length::{foot, meter},
};

mod data;
mod dubin;

mod drawing;

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

fn main() {
    let raw_aircrafts = std::fs::read_to_string("../../data/aircraft.yml").unwrap();
    let raw_locations = std::fs::read_to_string("../../data/locations.yml").unwrap();

    let aircrafts: Vec<Aircraft> = serde_yaml::from_str(&raw_aircrafts).unwrap();
    let locations: Vec<Location> = serde_yaml::from_str(&raw_locations).unwrap();

    let simulation = Simulation {
        bank: Angle::new::<degree>(45.0),
        resolution: Length::new::<meter>(10.0),
        altitudes: vec![
            Length::new::<foot>(1000.0),
            Length::new::<foot>(1500.0),
            Length::new::<foot>(2000.0),
            Length::new::<foot>(2300.0),
        ],
    };

    let output = Output::new(aircrafts, locations, simulation);
    let json = serde_json::to_string_pretty(&output).unwrap();
    let mut file = File::create("out/output.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
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
                        location_id: location.id,
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
            } else if landable_bearing == location.reverse_bearing() {
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
}
