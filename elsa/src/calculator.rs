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
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Map};
use std::{cmp::Ordering, collections::HashMap, f64::consts::FRAC_PI_2};
use strum::IntoEnumIterator;
use svg::{
    node::element::{path::Data, Path, Rectangle, Text},
    Document,
};
use uom::si::{
    angle::{degree, radian},
    f64::{Angle, Length},
    length::{foot, meter},
};
use wasm_bindgen::prelude::*;

pub struct AircraftRangeProfile([Point<f64>; 18]);
#[derive(Clone)]
pub struct LocationRangeProfile([Point<f64>; 18]);

#[wasm_bindgen(inspectable)]
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub bank: f64,
    pub epsilon: f64,

    #[wasm_bindgen(js_name = "unsafeLandingHeadroom")]
    pub unsafe_landing_headroom: f64,
    #[wasm_bindgen(js_name = "riskyLandingHeadroom")]
    pub risky_landing_headroom: f64,

    #[wasm_bindgen(js_name = "eventLocationClassification")]
    pub event_location_classification: RiskClassification,
    #[wasm_bindgen(js_name = "denselyCrowdedClassification")]
    pub densely_crowded_classification: RiskClassification,
}

#[wasm_bindgen(inspectable)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RiskAssessment {
    pub overall: RiskClassification,
    pub surface: RiskClassification,
    pub headroom: RiskClassification,
    pub humans: RiskClassification,
}

#[wasm_bindgen]
pub struct Calculator;

impl Calculator {
    fn aircraft_range_profile(
        &self,
        preferences: &Preferences,
        aircraft: &Aircraft,
        altitude: f64,
    ) -> AircraftRangeProfile {
        let maximum_range = aircraft.glide.ratio() * altitude * 2.0;
        let circle_radius = aircraft.glide.turn_radius(preferences.bank);
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

            let distance = binary_search(0.0, maximum_range, preferences.epsilon, |range| {
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
                            .height_loss_over_geometric_path(&path, preferences.bank)
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
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen(js_name = "locationHitboxes")]
    pub fn location_hitboxes(
        &self,
        location_map: &LocationMap,
        distance: f64,
    ) -> Result<String, JsValue> {
        let features = location_map
            .locations()
            .map(|location| {
                let centroid = location.centroid();
                let mut properties = Map::new();
                properties.insert("id".into(), location.id().into());
                properties.insert("lat".into(), centroid.lat().into());
                properties.insert("lng".into(), centroid.lng().into());

                Feature {
                    bbox: None,
                    geometry: Some((&location.spaced_polygon(distance)).into()),
                    id: Some(Id::String(location.id())),
                    properties: Some(properties),
                    foreign_members: None,
                }
            })
            .collect();

        let geojson = GeoJson::FeatureCollection(FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        });

        Ok(serde_json::to_string(&geojson).map_err(|e| e.to_string())?)
    }

    #[wasm_bindgen(js_name = assessRisk)]
    pub fn assess_risk(
        &self,
        preferences: &Preferences,
        location: &Location,
        aircraft: &Aircraft,
    ) -> RiskAssessment {
        use RiskClassification::*;

        // Step 1: Check surface type
        let surface = match location.surface {
            SurfaceType::Water => Unsafe,
            _ => Safe,
        };

        // Step 2: Verify landing headroom
        let headroom = match location.landing_headroom(aircraft) {
            headroom if headroom < preferences.unsafe_landing_headroom => Unsafe,
            headroom if headroom < preferences.risky_landing_headroom => Risky,
            _ => Safe,
        };

        // Step 3: Check for human presence
        let event_risky = preferences.event_location_classification == RiskClassification::Risky;
        let event_unsafe = preferences.event_location_classification == RiskClassification::Unsafe;
        let dense_risky = preferences.densely_crowded_classification == RiskClassification::Risky;
        let dense_unsafe = preferences.densely_crowded_classification == RiskClassification::Unsafe;

        let humans = match location.human_presence {
            HumanPresenceCategory::EventOnly if event_risky => Risky,
            HumanPresenceCategory::EventOnly if event_unsafe => Unsafe,
            HumanPresenceCategory::Dense if dense_risky => Risky,
            HumanPresenceCategory::Dense if dense_unsafe => Unsafe,
            _ => Safe,
        };

        // Step 4: Profit!
        RiskAssessment {
            overall: surface + headroom + humans,
            surface,
            headroom,
            humans,
        }
    }

    #[wasm_bindgen(js_name = reachabilityGeoJSON)]
    pub fn reachability_geojson(
        &self,
        preferences: &Preferences,
        location_map: &LocationMap,
        aircraft: &Aircraft,
        altitude: f64,
    ) -> Result<String, JsValue> {
        // Step 1: Calculate and cache the aircraft range profile
        let aircraft_range_profile = self.aircraft_range_profile(preferences, aircraft, altitude);

        // Step 2: Create polygons and assess risk for each location
        let polygons = location_map.locations().map(|location| {
            (
                self.assess_risk(preferences, location, aircraft).overall,
                self.location_range_polygon(location, aircraft, &aircraft_range_profile),
                location.id(),
            )
        });

        // Step 3: Group and union the polygons by risk and create individual features
        let (mut risk_map, feature_map): (
            HashMap<RiskClassification, MultiPolygon<f64>>,
            HashMap<String, Feature>,
        ) = polygons.fold(
            (HashMap::new(), HashMap::new()),
            |(mut risk_map, mut feature_map), (risk, polygon, id)| {
                // Create an individual geojson feature
                let mut properties = Map::new();
                properties.insert("id".into(), id.clone().into());
                properties.insert("risk".into(), to_value(risk).unwrap());
                let feature = Feature {
                    bbox: None,
                    geometry: Some((&polygon).into()),
                    id: None,
                    properties: Some(properties),
                    foreign_members: None,
                };
                feature_map.insert(id, feature);

                // Union the polygon with the corresponding risk category
                let polygon = match risk_map.remove(&risk) {
                    Some(existing_polygon) => existing_polygon.union(&polygon),
                    None => MultiPolygon(vec![polygon]),
                };

                risk_map.insert(risk, polygon);
                (risk_map, feature_map)
            },
        );

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

        let by_risk_geojson = GeoJson::FeatureCollection(FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        });

        let combined = serde_json::json!({
            "byRisk": by_risk_geojson,
            "byID": feature_map
        });

        Ok(serde_json::to_string(&combined).map_err(|e| e.to_string())?)
    }

    #[wasm_bindgen(js_name = locationGeoJSON)]
    pub fn location_geojson(
        &self,
        preferences: &Preferences,
        location_map: &LocationMap,
        aircraft: &Aircraft,
    ) -> String {
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
                    to_value(self.assess_risk(preferences, location, aircraft).overall).unwrap(),
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
        preferences: &Preferences,
        latitude: f64,
        longitude: f64,
        heading: f64,
        altitude: f64,
        aircraft: &Aircraft,
        locations: &LocationMap,
    ) -> String {
        let start = Point::new(longitude, latitude);
        let radius = Length::new::<meter>(aircraft.glide.turn_radius(preferences.bank));

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
                            .height_loss_over_geographic_path(&path, preferences.bank);

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
                    let risk = self.assess_risk(preferences, location, aircraft).overall;

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

    #[wasm_bindgen(js_name = takeoffProfile)]
    pub fn takeoff_profile(&self, aircraft: &Aircraft, distance: f64) -> String {
        let fifty_feet = 15.24;

        // Location geometry data
        let available_distance = distance; // 2270.0 D6; 2800 D8; 3800 A1;

        // Aircraft performance data
        let takeoff_ground_roll = aircraft.takeoff.ground_roll();
        let takeoff_total_dist = aircraft.takeoff.total_distance();

        let climb_slope = 1.0 / aircraft.climb.ratio();
        let descent_slope = -1.0 / aircraft.landing.descend_ratio();

        // TODO The ground roll is dependent on the surface (!)
        //      When the last ~50% of the ground roll is gras,
        //      those 50% should be multiplied by the corresponding factor!
        let landing_ground_roll = aircraft.landing.ground_roll();
        let landing_total_dist = aircraft.landing.total_distance();

        // Helpful point definitions
        let rotation_point = Point::new(takeoff_ground_roll, 0.0);
        let climb_point = Point::new(takeoff_total_dist, fifty_feet);

        let landing_point = Point::new(available_distance - landing_total_dist, fifty_feet);
        let touchdown_point = Point::new(available_distance - landing_ground_roll, 0.0);

        // Calculate the y-intercept for both lines
        let climb_y_intercept = -climb_slope * climb_point.x() + fifty_feet;
        let descent_y_intercept = -descent_slope * landing_point.x() + fifty_feet;

        // Calculate the intersection between the climb and descent line
        let intersection_x =
            (climb_y_intercept - descent_y_intercept) / (descent_slope - climb_slope);
        let intersection_y = climb_slope * intersection_x + climb_y_intercept;
        let intersection = Point::new(intersection_x, intersection_y);

        // Calculate the altitude when continuing the climb
        let climb_through_altitude = climb_slope * available_distance + climb_y_intercept;
        let climb_through_point = Point::new(available_distance, climb_through_altitude);

        log::debug!("Distn avail: {:?}", available_distance);
        log::debug!("Tkoff point: {:?}", rotation_point.x());
        log::debug!("Climb point: {:?}", climb_point);
        log::debug!("Lndg point:  {:?}", landing_point);
        log::debug!("Tchdn point: {:?}", touchdown_point.x());
        log::debug!(
            "Climb line equation: y = {} * x + {}",
            climb_slope,
            climb_y_intercept
        );
        log::debug!(
            "Desct line equation: y = {} * x + {}",
            descent_slope,
            descent_y_intercept
        );
        log::debug!("Intersection: {:?}", intersection);

        // Define the data for all lines
        let width = available_distance;
        let height = climb_through_altitude;
        let coordinates = |p: Point<f64>| (p.x(), height - p.y());

        let acceleration_line_data = Data::new()
            .move_to((0.0, height))
            .line_to(coordinates(rotation_point));

        let takeoff_line_data = Data::new()
            .move_to(coordinates(rotation_point))
            .line_to(coordinates(climb_point));

        let climb_line_data = Data::new()
            .move_to(coordinates(climb_point))
            .line_to(coordinates(climb_through_point));

        let descent_line_data = Data::new()
            .move_to(coordinates(intersection))
            .line_to(coordinates(landing_point));

        let landing_line_data = Data::new()
            .move_to(coordinates(landing_point))
            .line_to(coordinates(touchdown_point));

        let deceleration_line_data = Data::new()
            .move_to(coordinates(touchdown_point))
            .line_to((available_distance, height));

        let height_mark_data = Data::new()
            .move_to((0.0, height - intersection_y))
            .horizontal_line_to(width);

        // Build the paths
        let risk_color = "#FF3D00";
        let asphalt_color = "#607D8B";
        let route_color = "#2196F3";

        let build_path = |data: Data, color: &'static str| {
            Path::new()
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 2)
                .set("d", data)
        };

        let descent_line = build_path(descent_line_data, "red").set("stroke-dasharray", "10 5");
        let landing_line = build_path(landing_line_data, "purple").set("stroke-dasharray", "10 5");

        let height_marker = build_path(height_mark_data, risk_color)
            .set("stroke-dasharray", "15 8")
            .set("stroke-opacity", "0.5")
            .set("class", "label")
            .set("label", "Hello world!");

        let height_label = format!(
            "{}m / {}ft",
            intersection_y.round(),
            Length::new::<meter>(intersection_y).get::<foot>().round()
        );
        let height_text = Text::new()
            .set("x", 10)
            .set("dy", 10)
            .set("alignment-baseline", "hanging")
            .set(
                "y",
                format!("{}%", (height - intersection_y) / height * 100.0),
            )
            .add(svg::node::Text::new(height_label));

        let danger_rect = Rectangle::new()
            .set("fill", risk_color)
            .set("fill-opacity", "0.25")
            .set("x", 0.0)
            .set("y", 0.0)
            .set("width", width)
            .set("height", height - intersection_y);

        // Assemble everything into an SVG
        let graph = Document::new()
            .set("viewBox", (0.0, 0.0, width, height))
            .set("preserveAspectRatio", "none")
            .add(danger_rect)
            .add(height_marker)
            .add(build_path(acceleration_line_data, asphalt_color))
            .add(build_path(takeoff_line_data, "green"))
            .add(build_path(climb_line_data, route_color))
            .add(descent_line)
            .add(landing_line)
            .add(build_path(deceleration_line_data, asphalt_color));

        let wrapper = Document::new().add(graph).add(height_text);

        wrapper.to_string()
    }
}

#[wasm_bindgen]
impl Preferences {
    #[wasm_bindgen(constructor)]
    pub fn new(serialized: String) -> Result<Preferences, JsValue> {
        Ok(serde_json::from_str(&serialized).map_err(|e| e.to_string())?)
    }

    pub fn serialize(&self) -> Result<String, JsValue> {
        Ok(serde_json::to_string(&self).map_err(|e| e.to_string())?)
    }
}
