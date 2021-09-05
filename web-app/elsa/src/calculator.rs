use crate::{
    dubin::{
        calculate_dubin_path_candidates, calculate_georeferenced_dubin_path_candidates,
        GeographicDubinPath,
    },
    helpers::binary_search,
    Aircraft, HumanPresenceCategory, Location, LocationMap, RiskClassification, SurfaceType,
    UsageType,
};
use geo::{
    prelude::{EuclideanDistance, HaversineDestination},
    rotate::RotatePoint,
    LineString, MultiLineString, MultiPolygon, Point, Polygon,
};
use geo_booleanop::boolean::BooleanOp;
use geojson::{feature::Id, Feature, FeatureCollection, GeoJson};
use serde_json::{to_value, Map};
use std::{cmp::Ordering, collections::HashMap, f64::consts::FRAC_PI_2};
use strum::IntoEnumIterator;
use uom::si::{
    angle::{degree, radian},
    f64::{Angle, Length},
    length::meter,
};
use wasm_bindgen::prelude::*;

pub struct AircraftRangeProfile([Point<f64>; 18]);
#[derive(Clone)]
pub struct LocationRangeProfile([Point<f64>; 18]);

#[wasm_bindgen(inspectable)]
pub struct Preferences {
    pub bank: f64,
    pub epsilon: f64,

    pub unsafe_landing_headroom: f64,
    pub risky_landing_headroom: f64,
}

#[wasm_bindgen]
pub struct Calculator {
    preferences: Preferences,
}

impl Calculator {
    fn aircraft_range_profile(&self, aircraft: &Aircraft, altitude: f64) -> AircraftRangeProfile {
        let maximum_range = aircraft.glide.ratio() * altitude * 2.0;
        let circle_radius = aircraft.glide.turn_radius(self.preferences.bank);
        let circle_origin = Point::new(-circle_radius, 0.0);

        let point_on_circle = |angle: f64| {
            circle_origin + Point::new(circle_radius * angle.cos(), circle_radius * angle.sin())
        };

        let origin = Point::new(0.0, 0.0);
        let origin_angle = Angle::new::<degree>(90.0);
        let radius = Length::new::<meter>(circle_radius);

        let mut points = [Point::new(0.0, 0.0); 18];

        // Beyond 180º the "worst" angle can no longer be derived by looking at the approach circle on the side of the aircraft location only.
        // As the angle grows, it becomes viable to choose the other side for the approach and thus the actual worst angle is shallower.
        // Calculating this is a bit more involved, thus we will ignore this detail and only calculate angles up to 180º. Beyond that,
        // a straight connection line will be drawn. While this is only an approximation, it errors on the side of caution and thus presents
        // an acceptable trade-off between code complexity, computational load, and personal sanity.
        //
        // Besides, the error is less than 100m and only ever applies to non-reversible locations of which there are not many. Soooo ... 🤷‍♂️
        for angle_deg in 0..18 {
            let angle = (-(angle_deg as f64) * 10.0).to_radians();
            let ray_origin = point_on_circle(angle);
            let ray = angle - FRAC_PI_2;
            let ray_typed = Angle::new::<radian>(ray);

            let distance = binary_search(0.0, maximum_range, self.preferences.epsilon, |range| {
                let ray_target = ray_origin + Point::new(range * ray.cos(), range * ray.sin());

                let path_candidates = calculate_dubin_path_candidates(
                    ray_target,
                    origin,
                    // TODO Make it an option to "invert" the start angle and thus get a "best case" range. Just out of curiosity on how much it changes the ranges :P
                    ray_typed,
                    origin_angle,
                    radius,
                );

                let minimum_height_loss = path_candidates
                    .into_iter()
                    .map(|path| {
                        aircraft
                            .glide
                            .height_loss_over_geometric_path(&path, self.preferences.bank)
                    })
                    .min_by(|x, y| x.partial_cmp(y).unwrap());

                minimum_height_loss.map(|h| h <= altitude).unwrap_or(true)
            });

            points[angle_deg] = ray_origin + Point::new(distance * ray.cos(), distance * ray.sin());
        }

        AircraftRangeProfile(points)
    }

    fn location_range_profile(
        &self,
        location: &Location,
        aircraft: &Aircraft,
        aircraft_range_profile: &AircraftRangeProfile,
    ) -> LocationRangeProfile {
        let inset = location.inset(aircraft);
        let length = location.length();

        let mut points = [Point::new(0.0, 0.0); 18];

        // Step 1: Copy over the first 90º of points
        for i in 0..9 {
            points[i] = aircraft_range_profile.0[i];
        }

        // Step 2.1: If the location is NOT reversible, copy over and inset the second 90º of points
        if !location.reversible {
            for i in 0..9 {
                let mut point = aircraft_range_profile.0[17 - i].clone();
                point.0.y += inset;
                points[17 - i] = point;
            }
        }
        // Step 2.2: If the location IS reversible, flip, copy, and offset the first 90º of points
        else {
            for i in 0..9 {
                let mut point = aircraft_range_profile.0[i].clone();
                point.0.y *= -1.0;
                point.0.y += length;
                points[17 - i] = point;
            }
        }

        LocationRangeProfile(points)
    }

    fn location_range_polygon(
        &self,
        location: &Location,
        aircraft: &Aircraft,
        aircraft_range_profile: &AircraftRangeProfile,
    ) -> Polygon<f64> {
        // Step 0: Generate a range profile
        let profile = self.location_range_profile(location, aircraft, aircraft_range_profile);

        // Step 1: Mirror the profile along the Y-axis to cover the full 360º
        let mut mirrored_profile = LocationRangeProfile([Point::new(0.0, 0.0); 18]);

        for i in 0..18 {
            mirrored_profile.0[i] = profile.0[17 - i];
            mirrored_profile.0[i].0.x *= -1.0;
        }

        // Step 2: Create the points for a LineString from the two sides,
        //         rotate it to match the locations heading,
        //         and convert from relative geometrics points to absolute geographic coordinates.
        let origin = Point::new(0.0, 0.0);
        let points = profile
            .0
            .iter()
            .chain(mirrored_profile.0.iter())
            .map(|p| {
                let rotated_point = p.rotate_around_point(location.bearing(), origin);
                let bearing = rotated_point.y().atan2(rotated_point.x()).to_degrees() - 90.0;
                let distance = rotated_point.euclidean_distance(&origin);

                location.start().haversine_destination(bearing, distance).0
            })
            .collect::<Vec<_>>();

        // Step 3: Convert it into a polygon and profit!
        Polygon::new(LineString(points), vec![])
    }
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new(preferences: Preferences) -> Self {
        Self { preferences }
    }

    #[wasm_bindgen(js_name = assessRisk)]
    pub fn assess_risk(&self, location: &Location, aircraft: &Aircraft) -> RiskClassification {
        let mut risky = false;
        let mut deadly = false;

        // Step 1: Check surface type
        match location.surface {
            SurfaceType::Water => deadly = true,
            _ => {}
        }

        // Step 2: Verify landing headroom
        match location.landing_headroom(aircraft) {
            headroom if headroom < self.preferences.unsafe_landing_headroom => deadly = true,
            headroom if headroom < self.preferences.risky_landing_headroom => risky = true,
            _ => {}
        }

        // Step 3: Check for human presence
        match location.human_presence {
            HumanPresenceCategory::EventOnly => risky = true,
            HumanPresenceCategory::Dense => risky = true,
            _ => {}
        }

        // Step 4: Profit!
        if deadly {
            RiskClassification::Unsafe
        } else if risky {
            RiskClassification::Risky
        } else {
            RiskClassification::Safe
        }
    }

    #[wasm_bindgen(js_name = reachabilityGeoJSON)]
    pub fn reachability_geojson(
        &self,
        location_map: &LocationMap,
        aircraft: &Aircraft,
        altitude: f64,
    ) -> Result<String, JsValue> {
        // Step 1: Calculate and cache the aircraft range profile
        let aircraft_range_profile = self.aircraft_range_profile(aircraft, altitude);

        // Step 2: Create polygons and assess risk for each location
        let polygons = location_map.locations().map(|location| {
            (
                self.assess_risk(location, aircraft),
                self.location_range_polygon(location, aircraft, &aircraft_range_profile),
            )
        });

        // Step 3: Group and union the polygons by risk
        let mut risk_map: HashMap<RiskClassification, MultiPolygon<f64>> =
            polygons.fold(HashMap::new(), |mut acc, (risk, polygon)| {
                let polygon = match acc.remove(&risk) {
                    Some(existing_polygon) => existing_polygon.union(&polygon),
                    None => MultiPolygon(vec![polygon]),
                };

                acc.insert(risk, polygon);
                acc
            });

        // Step 4: Subtract lower risk polygons from higher risk ones so they do not overlap on the map
        RiskClassification::iter().fold(vec![], |mut less_risky_polygons, risk| {
            match risk_map.get_mut(&risk) {
                Some(polygon) => {
                    let original_polygon = polygon.clone();
                    less_risky_polygons.iter().for_each(|less_risky_polygon| {
                        *polygon = polygon.difference(less_risky_polygon);
                    });
                    less_risky_polygons.push(original_polygon);
                    less_risky_polygons
                }
                None => less_risky_polygons,
            }
        });

        // Step 5: Convert into a vector of features with an associated risk property
        let features = risk_map
            .into_iter()
            .map(|(risk, polygon)| {
                let mut properties = Map::new();
                properties.insert(String::from("risk"), to_value(risk).unwrap());

                Feature {
                    bbox: None,
                    geometry: Some((&polygon).into()),
                    id: None,
                    properties: Some(properties),
                    foreign_members: None,
                }
            })
            .collect::<Vec<_>>();

        let geojson = GeoJson::FeatureCollection(FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        });

        Ok(serde_json::to_string(&geojson).map_err(|e| e.to_string())?)
    }

    #[wasm_bindgen(js_name = locationGeoJSON)]
    pub fn location_geojson(&self, location_map: &LocationMap, aircraft: &Aircraft) -> String {
        let features = location_map
            .locations()
            .filter(|location| location.usage != UsageType::Aeronautical)
            .map(|location| {
                let mut line =
                    MultiLineString(vec![LineString(vec![location.start().0, location.end().0])]);

                // If the location is NOT reversible, add a perpendicular dash at the end
                // (to match the style of that one symbol airports can put out in case of radio failure :P)
                if !location.reversible {
                    let bearing = location.bearing();
                    let stop_line_start =
                        location.end().haversine_destination(bearing + 90.0, 15.0).0;
                    let stop_line_end =
                        location.end().haversine_destination(bearing - 90.0, 15.0).0;
                    let stop_line = LineString(vec![stop_line_start, stop_line_end]);

                    line.0.push(stop_line);
                }

                let mut properties = Map::new();
                properties.insert(
                    String::from("risk"),
                    to_value(self.assess_risk(location, aircraft)).unwrap(),
                );

                Feature {
                    bbox: None,
                    geometry: Some((&line).into()),
                    id: Some(Id::String(location.id())),
                    properties: Some(properties),
                    foreign_members: None,
                }
            })
            .collect::<Vec<_>>();

        let geojson = GeoJson::FeatureCollection(FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        });

        geojson.to_string()
    }

    #[wasm_bindgen(js_name = landingOptions)]
    pub fn landing_options(
        &self,
        latitude: f64,
        longitude: f64,
        heading: f64,
        altitude: f64,
        aircraft: &Aircraft,
        locations: &LocationMap,
    ) -> String {
        let start = Point::new(longitude, latitude);
        let radius = Length::new::<meter>(aircraft.glide.turn_radius(self.preferences.bank));

        let features = locations
            .locations()
            .filter_map(|location| {
                let mut points = vec![(location.start(), location.bearing())];
                if location.reversible {
                    points.push((location.end(), location.reverse_bearing()));
                }

                points
                    .into_iter()
                    .flat_map(|(end, target_heading)| {
                        let start_bearing = Angle::new::<degree>(heading);
                        let end_bearing = Angle::new::<degree>(target_heading);

                        calculate_georeferenced_dubin_path_candidates(
                            start,
                            end,
                            start_bearing,
                            end_bearing,
                            radius,
                        )
                    })
                    .map(|path| {
                        let height_loss = aircraft
                            .glide
                            .height_loss_over_geographic_path(&path, self.preferences.bank);

                        (path, height_loss, location)
                    })
                    .filter(|(_, height_loss, _)| *height_loss < altitude)
                    .min_by(|(_, height_loss_a, _), (_, height_loss_b, _)| {
                        height_loss_a
                            .partial_cmp(&height_loss_b)
                            .unwrap_or(Ordering::Equal)
                    })
            })
            .map(
                |(path, height_loss, location): (GeographicDubinPath, f64, &Location)| {
                    let points = path.points().map(|p| p.0).collect::<Vec<_>>();
                    let line = LineString(points);
                    let risk = self.assess_risk(location, aircraft);

                    let mut properties = Map::new();
                    properties.insert(String::from("risk"), to_value(risk).unwrap());
                    properties.insert(String::from("heightLoss"), to_value(height_loss).unwrap());

                    Feature {
                        bbox: None,
                        geometry: Some((&line).into()),
                        id: Some(Id::String(location.id())),
                        properties: Some(properties),
                        foreign_members: None,
                    }
                },
            )
            .collect::<Vec<_>>();

        let geojson = GeoJson::FeatureCollection(FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        });

        geojson.to_string()
    }
}

#[wasm_bindgen]
impl Preferences {
    #[wasm_bindgen(constructor)]
    pub fn new(serialized: Option<String>) -> Self {
        match serialized {
            Some(_serialized) => todo!(),
            None => Self::default(),
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            bank: 45.0f64.to_radians(),
            epsilon: 0.1,

            unsafe_landing_headroom: -0.15,
            risky_landing_headroom: -0.05,
        }
    }
}
