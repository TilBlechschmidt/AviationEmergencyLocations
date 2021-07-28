#![allow(dead_code)]

use std::thread;

use dubin::calculate_dubin_path_candidates;
use image::{GrayImage, Luma, Rgb, RgbImage};
use imageproc::{
    contours::find_contours,
    drawing::{draw_cross_mut, draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut},
};
use rusttype::{Font, Scale};
use sim::{build_simulation, Simulation};
use uom::si::{
    angle::degree,
    f64::*,
    length::{foot, meter},
};

// Units:
// - Angle: Radians
// - Distance: m
// - Height: m
// - Speed: m/s
// - Slope: Multiplying by height gives ground distance

type Slope = f64;
type Altitude = Length;
type Distance = Length;
type Location = (Distance, Distance);

mod dubin;
mod data;
mod sim;

use dubin::*;

fn main() {
    // let sim = build_simulation();
    // let (image, mask) = simulation_fused_field_map(&sim, 2300.0);
    // image.save("render.tiff").unwrap();

    // let contours = find_contours::<i32>(&mask);
    // let contour = contours.first().unwrap();
    // println!(
    //     "{:?} contour points, type: {:?}",
    //     contour.points.len(),
    //     contour.border_type
    // );

    batch_altitude_fused_field_map();
}

fn batch_altitude_fused_field_map() {
    let mut handles = Vec::new();

    for i in 1..26 {
        let sim = build_simulation();
        let handle = thread::spawn(move || {
            let altitude = 100.0 * (i as f64);
            println!("Calculating {}ft", altitude);
            let (image, _) = simulation_fused_field_map(&sim, altitude);
            image.save(format!("out/{:0>2}.tiff", i)).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn batch_field_map() {
    for i in 0..36 {
        let angle = (i as f64) * 10.0;
        println!("Calculating {}º", angle);
        let image = field_map(angle);
        image.save(format!("out/{:0>2}.tiff", i)).unwrap();
    }
}

fn altitude_loss_over_path(sim: &Simulation, path: &DubinPath) -> Distance {
    match path {
        DubinPath::CSC(arc1, straight, arc2) => {
            sim.altitude_lost_in_turn(arc1.angle())
                + sim.altitude_lost_over_distance(straight.length())
                + sim.altitude_lost_in_turn(arc2.angle())
        }
        DubinPath::CCC(arc1, arc2, arc3) => {
            sim.altitude_lost_in_turn(arc1.angle())
                + sim.altitude_lost_in_turn(arc2.angle())
                + sim.altitude_lost_in_turn(arc3.angle())
        }
    }
}

fn simulation_fused_field_map(sim: &Simulation, altitude: f64) -> (RgbImage, GrayImage) {
    let resolution = 500;
    let meters_per_pixel = 25;

    let mut image = RgbImage::new(resolution, resolution);
    let mut mask = GrayImage::new(resolution, resolution);

    let radius = sim.turn_radius();
    let end = (Distance::new::<meter>(0.0), Distance::new::<meter>(0.0));
    let end_angle = Angle::new::<degree>(0.0);

    for (p_x, p_y, pixel) in image.enumerate_pixels_mut() {
        let x = (p_x as i32 - resolution as i32 / 2) * meters_per_pixel;
        let y = (p_y as i32 - resolution as i32 / 2) * meters_per_pixel;

        let start = (
            Distance::new::<meter>(x as f64),
            Distance::new::<meter>(y as f64),
        );

        // Check if the given pixel is reachable from all start headings
        let check_reachable = |start_angle: Angle| {
            let candidates =
                calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);

            let shortest_path = candidates
                .into_iter()
                .reduce(|a, b| {
                    if altitude_loss_over_path(&sim, &a) < altitude_loss_over_path(&sim, &b) {
                        a
                    } else {
                        b
                    }
                })
                .unwrap();

            altitude_loss_over_path(&sim, &shortest_path).get::<foot>() <= altitude
        };

        // Check in 10º increments if it is always reachable or sometimes reachable
        let mut reachable = true;
        let mut sometimes_reachable = false;
        for angle in 0..360 {
            let start_angle = Angle::new::<degree>(angle as f64);
            let in_range = check_reachable(start_angle);

            reachable = reachable && in_range;
            sometimes_reachable = sometimes_reachable || in_range;
        }

        // Explicitly check if it is reachable when heading straight away from it (e.g. during takeoff)
        let opposite_direction_reachable = check_reachable(end_angle + Angle::new::<degree>(180.0));

        // Write colored pixels to human-readable version
        if reachable {
            pixel.0[1] = 255;
        } else if opposite_direction_reachable {
            pixel.0[2] = 255;
        } else if sometimes_reachable {
            pixel.0[0] = 255;
            pixel.0[1] = 255;
        } else {
            pixel.0[0] = 0;
            pixel.0[1] = 0;
            pixel.0[2] = 0;
        }

        // Write only "always reachable" state to mask
        if reachable {
            mask.put_pixel(p_x, p_y, Luma([255]));
        }
    }

    draw_label(&format!("{}ft", altitude), &mut image);

    (image, mask)
}

fn field_map(angle: f64) -> RgbImage {
    let mut image = RgbImage::new(1000, 1000);

    let center = (500, 500);
    let radius = Distance::new::<meter>(50.0);
    let end = (
        Distance::new::<meter>(center.0 as f64),
        Distance::new::<meter>(center.1 as f64),
    );

    let end_angle = Angle::new::<degree>(0.0);
    let start_angle = Angle::new::<degree>(angle);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let start = (
            Distance::new::<meter>(x as f64),
            Distance::new::<meter>(y as f64),
        );

        let candidates =
            calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);

        let shortest_path = candidates
            .into_iter()
            .reduce(|a, b| if a.length() < b.length() { a } else { b })
            .unwrap();

        let length = ((1.0 - shortest_path.length().get::<meter>() / 400.0) * 255.0).round() as u8;
        pixel.0[0] = length;
    }

    draw_cross_mut(&mut image, Rgb([0u8, 255u8, 0u8]), center.0, center.1);

    // let start = (Distance::new::<meter>(500.0), Distance::new::<meter>(450.0));
    // let candidates = calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);
    // let shortest_path = candidates
    //     .into_iter()
    //     .reduce(|a, b| if a.length() < b.length() { a } else { b })
    //     .unwrap();

    // draw_dubin_path(shortest_path, &mut image);

    draw_label(&format!("{}º", angle), &mut image);

    image
}

fn draw_label(label: &str, image: &mut RgbImage) {
    let font_data: &[u8] = include_bytes!("./DejaVuSansMono.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    let x = image.width() / 2 - (label.len() as u32 * 15) / 2;
    let y = image.height() - 100;
    draw_text_mut(
        image,
        Rgb([255, 255, 255]),
        x,
        y,
        Scale::uniform(20.0),
        &font,
        label,
    );
}

fn example_dubin_path() {
    let mut image = RgbImage::new(250, 250);

    let center = (125, 125);
    let radius = Distance::new::<meter>(25.0);
    let end = (
        Distance::new::<meter>(center.0 as f64),
        Distance::new::<meter>(center.1 as f64),
    );

    let end_angle = Angle::new::<degree>(0.0);
    let start_angle = Angle::new::<degree>(180.0);

    let start = (Distance::new::<meter>(125.0), Distance::new::<meter>(140.0));
    let candidates = calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);
    let shortest_path = candidates
        .into_iter()
        .reduce(|a, b| if a.length() < b.length() { a } else { b })
        .unwrap();

    draw_dubin_path(shortest_path, &mut image);

    image.save("render.tiff").unwrap();
}

fn draw_arc(arc: &Arc, image: &mut RgbImage) {
    let circle_color = Rgb([99u8, 99u8, 99u8]);
    let cross_color = Rgb([0u8, 255u8, 0u8]);
    let text_color = Rgb([255u8, 255u8, 255u8]);
    let arc_color = Rgb([0u8, 0u8, 255u8]);
    let font_data: &[u8] = include_bytes!("./DejaVuSansMono.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let cx = arc.circle.center().0.get::<meter>();
    let cy = arc.circle.center().1.get::<meter>();
    let r = arc.circle.radius().get::<meter>();

    // Draw the circle
    draw_hollow_circle_mut(image, (cx as i32, cy as i32), r as i32, circle_color);

    // Calculate the starting angle and delta
    let start_angle = arc.start_angle().get::<degree>();
    let theta = arc.angle().get::<degree>() as i32;
    let sign = if arc.circle.direction == Direction::Left {
        1.0
    } else {
        -1.0
    };

    // Draw a pixel for every degree
    for angle in 0..theta.abs() {
        let rotation = (start_angle + sign * (angle as f64)).to_radians();

        let x = (cx + r * rotation.cos()).round();
        let y = (cy + r * rotation.sin()).round();
        image.put_pixel(x as u32, y as u32, arc_color);
    }

    // Mark the start and end of the arc
    draw_cross_mut(
        image,
        cross_color,
        arc.start().0.get::<meter>() as i32,
        arc.start().1.get::<meter>() as i32,
    );
    draw_cross_mut(
        image,
        cross_color,
        arc.end().0.get::<meter>() as i32,
        arc.end().1.get::<meter>() as i32,
    );

    // Label the arc angle
    draw_text_mut(
        image,
        text_color,
        arc.circle.center().0.get::<meter>() as u32 - 10,
        arc.circle.center().1.get::<meter>() as u32 - 5,
        Scale::uniform(10.0),
        &font,
        &format!("{}º", arc.angle().get::<degree>().round()),
    );
}

fn draw_tangent(tangent: &Tangent, image: &mut RgbImage) {
    let line_color = Rgb([0u8, 0u8, 255u8]);
    draw_line_segment_mut(
        image,
        (
            tangent.start().0.get::<meter>() as f32,
            tangent.start().1.get::<meter>() as f32,
        ),
        (
            tangent.end().0.get::<meter>() as f32,
            tangent.end().1.get::<meter>() as f32,
        ),
        line_color,
    );
}

fn draw_dubin_path(path: DubinPath, image: &mut RgbImage) {
    let length = path.length();
    let name = path.name();

    let lengths = match path {
        DubinPath::CSC(arc1, tangent, arc2) => {
            draw_arc(&arc1, image);
            draw_arc(&arc2, image);
            draw_tangent(&tangent, image);
            format!(
                "{}+{}+{}",
                arc1.length().get::<meter>().round(),
                tangent.length().get::<meter>().round(),
                arc2.length().get::<meter>().round(),
            )
        }
        DubinPath::CCC(arc1, arc2, arc3) => {
            draw_arc(&arc1, image);
            draw_arc(&arc2, image);
            draw_arc(&arc3, image);
            format!(
                "{}+{}+{}",
                arc1.length().get::<meter>().round(),
                arc2.length().get::<meter>().round(),
                arc3.length().get::<meter>().round(),
            )
        }
    };

    // Labels
    let text_color = Rgb([255u8, 255u8, 255u8]);
    let font_data: &[u8] = include_bytes!("./DejaVuSansMono.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    let description = format!("{} {}={}px", name, lengths, length.get::<meter>().round());
    draw_text_mut(
        image,
        text_color,
        10,
        image.height() - 25,
        Scale::uniform(15.0),
        &font,
        &description,
    );
}
