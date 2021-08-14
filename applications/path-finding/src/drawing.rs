#![allow(dead_code)]

use std::thread;

use geo::{point, Point};
use image::{GrayImage, Luma, Rgb, RgbImage};
use imageproc::{
    contours::find_contours,
    drawing::{draw_cross_mut, draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut},
};
use rusttype::{Font, Scale};
// use sim::{build_simulation, Simulation};
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

// mod dubin;
// mod data;
// mod sim;

use crate::data::Aircraft;

use super::dubin::*;

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

    // batch_altitude_fused_field_map();
}

// fn batch_altitude_fused_field_map() {
//     let mut handles = Vec::new();

//     for i in 1..26 {
//         let sim = build_simulation();
//         let handle = thread::spawn(move || {
//             let altitude = 100.0 * (i as f64);
//             println!("Calculating {}ft", altitude);
//             let (image, _) = simulation_fused_field_map(&sim, altitude);
//             image.save(format!("out/{:0>2}.tiff", i)).unwrap();
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

// fn batch_field_map() {
//     for i in 0..36 {
//         let angle = (i as f64) * 10.0;
//         println!("Calculating {}º", angle);
//         let image = field_map(angle);
//         image.save(format!("out/{:0>2}.tiff", i)).unwrap();
//     }
// }

fn altitude_loss_over_path(aircraft: &Aircraft, path: &DubinPath) -> Distance {
    let bank = Angle::new::<degree>(45.0);
    match path {
        DubinPath::CSC(arc1, straight, arc2) => {
            aircraft.glide.height_lost_for_turn(bank, arc1.angle())
                + aircraft
                    .glide
                    .height_lost_for_ground_track(straight.length())
                + aircraft.glide.height_lost_for_turn(bank, arc2.angle())
        }
        DubinPath::CCC(arc1, arc2, arc3) => {
            aircraft.glide.height_lost_for_turn(bank, arc1.angle())
                + aircraft.glide.height_lost_for_turn(bank, arc2.angle())
                + aircraft.glide.height_lost_for_turn(bank, arc3.angle())
        }
    }
}

pub fn simulation_fused_field_map(aircraft: &Aircraft, altitude: f64) -> (RgbImage, GrayImage) {
    let resolution = 500;
    let meters_per_pixel = 25;

    let mut image = RgbImage::new(resolution, resolution);
    let mut mask = GrayImage::new(resolution, resolution);

    let bank = Angle::new::<degree>(45.0);
    let radius = aircraft.glide.turn_radius(bank);
    let end = Point::new(0.0, 0.0);
    let end_angle = Angle::new::<degree>(0.0);

    for (p_x, p_y, pixel) in image.enumerate_pixels_mut() {
        let x = (p_x as i32 - resolution as i32 / 2) * meters_per_pixel;
        let y = (p_y as i32 - resolution as i32 / 2) * meters_per_pixel;

        let start = Point::new(x as f64, y as f64);

        // Check if the given pixel is reachable from all start headings
        let check_reachable = |start_angle: Angle| {
            let candidates =
                calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);

            let shortest_path = candidates
                .into_iter()
                .reduce(|a, b| {
                    if altitude_loss_over_path(&aircraft, &a)
                        < altitude_loss_over_path(&aircraft, &b)
                    {
                        a
                    } else {
                        b
                    }
                })
                .unwrap();

            altitude_loss_over_path(&aircraft, &shortest_path).get::<foot>() <= altitude
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

// fn field_map(angle: f64) -> RgbImage {
//     let mut image = RgbImage::new(1000, 1000);

//     let center = (500, 500);
//     let radius = Distance::new::<meter>(50.0);
//     let end = (
//         Distance::new::<meter>(center.0 as f64),
//         Distance::new::<meter>(center.1 as f64),
//     );

//     let end_angle = Angle::new::<degree>(0.0);
//     let start_angle = Angle::new::<degree>(angle);

//     for (x, y, pixel) in image.enumerate_pixels_mut() {
//         let start = (
//             Distance::new::<meter>(x as f64),
//             Distance::new::<meter>(y as f64),
//         );

//         let candidates =
//             calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);

//         let shortest_path = candidates
//             .into_iter()
//             .reduce(|a, b| if a.length() < b.length() { a } else { b })
//             .unwrap();

//         let length = ((1.0 - shortest_path.length().get::<meter>() / 400.0) * 255.0).round() as u8;
//         pixel.0[0] = length;
//     }

//     draw_cross_mut(&mut image, Rgb([0u8, 255u8, 0u8]), center.0, center.1);

//     // let start = (Distance::new::<meter>(500.0), Distance::new::<meter>(450.0));
//     // let candidates = calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);
//     // let shortest_path = candidates
//     //     .into_iter()
//     //     .reduce(|a, b| if a.length() < b.length() { a } else { b })
//     //     .unwrap();

//     // draw_dubin_path(shortest_path, &mut image);

//     draw_label(&format!("{}º", angle), &mut image);

//     image
// }

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
    let end = point!(
        x: center.0 as f64,
        y: center.1 as f64
    );

    let end_angle = Angle::new::<degree>(0.0);
    let start_angle = Angle::new::<degree>(180.0);

    let start = point!(x: 125.0, y: 140.0);
    let candidates = calculate_dubin_path_candidates(start, end, start_angle, end_angle, radius);
    let shortest_path = candidates
        .into_iter()
        .reduce(|a, b| if a.length() < b.length() { a } else { b })
        .unwrap();

    draw_dubin_path(shortest_path, &mut image, [0, 0]);

    image.save("render.tiff").unwrap();
}

fn draw_arc(arc: &Arc, image: &mut RgbImage, translation: [i32; 2]) {
    let circle_color = Rgb([99u8, 99u8, 99u8]);
    let cross_color = Rgb([255u8, 255u8, 0u8]);
    let text_color = Rgb([255u8, 255u8, 255u8]);
    let arc_color = Rgb([0u8, 0u8, 255u8]);
    let font_data: &[u8] = include_bytes!("./DejaVuSansMono.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let cx = arc.circle.center().0.get::<meter>();
    let cy = arc.circle.center().1.get::<meter>();
    let r = arc.circle.radius().get::<meter>();

    // Draw the circle
    draw_hollow_circle_mut(
        image,
        (cx as i32 + translation[0], cy as i32 + translation[1]),
        r as i32,
        circle_color,
    );

    // Draw the arc
    for point in arc.points() {
        let x = point.x() + translation[0] as f64;
        let y = point.y() + translation[1] as f64;
        image.put_pixel(x as u32, y as u32, arc_color);
    }

    // Mark the start and end of the arc
    draw_cross_mut(
        image,
        cross_color,
        arc.start().0.get::<meter>() as i32 + translation[0],
        arc.start().1.get::<meter>() as i32 + translation[1],
    );
    draw_cross_mut(
        image,
        cross_color,
        arc.end().0.get::<meter>() as i32 + translation[0],
        arc.end().1.get::<meter>() as i32 + translation[1],
    );

    // Label the arc angle
    draw_text_mut(
        image,
        text_color,
        (arc.circle.center().0.get::<meter>() - 10.0 + translation[0] as f64) as u32,
        (arc.circle.center().1.get::<meter>() - 5.0 + translation[1] as f64) as u32,
        Scale::uniform(10.0),
        &font,
        &format!("{}º", arc.angle().get::<degree>().round()),
    );
}

fn draw_tangent(tangent: &Tangent, image: &mut RgbImage, translation: [i32; 2]) {
    let line_color = Rgb([0u8, 0u8, 255u8]);
    draw_line_segment_mut(
        image,
        (
            tangent.start().0.get::<meter>() as f32 + translation[0] as f32,
            tangent.start().1.get::<meter>() as f32 + translation[1] as f32,
        ),
        (
            tangent.end().0.get::<meter>() as f32 + translation[0] as f32,
            tangent.end().1.get::<meter>() as f32 + translation[1] as f32,
        ),
        line_color,
    );
}

fn mark_start_and_end(start_arc: &Arc, end_arc: &Arc, image: &mut RgbImage, translation: [i32; 2]) {
    draw_cross_mut(
        image,
        Rgb([255, 0, 0]),
        start_arc.start().0.get::<meter>() as i32 + translation[0],
        start_arc.start().1.get::<meter>() as i32 + translation[1],
    );
    draw_cross_mut(
        image,
        Rgb([0, 255, 0]),
        end_arc.end().0.get::<meter>() as i32 + translation[0],
        end_arc.end().1.get::<meter>() as i32 + translation[1],
    );
}

pub fn draw_dubin_path(path: DubinPath, image: &mut RgbImage, translation: [i32; 2]) {
    let length = path.length();
    let name = path.name();

    let lengths = match path {
        DubinPath::CSC(arc1, tangent, arc2) => {
            draw_arc(&arc1, image, translation);
            draw_arc(&arc2, image, translation);
            draw_tangent(&tangent, image, translation);
            mark_start_and_end(&arc1, &arc2, image, translation);
            format!(
                "{}+{}+{}",
                arc1.length().get::<meter>().round(),
                tangent.length().get::<meter>().round(),
                arc2.length().get::<meter>().round(),
            )
        }
        DubinPath::CCC(arc1, arc2, arc3) => {
            draw_arc(&arc1, image, translation);
            draw_arc(&arc2, image, translation);
            draw_arc(&arc3, image, translation);
            mark_start_and_end(&arc1, &arc3, image, translation);
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
