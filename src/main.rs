extern crate cairo;
extern crate nalgebra;

mod actor;
mod draw;
mod road;

use std::{f64::consts::PI, fs::File, collections::HashSet};

use cairo::{Format, ImageSurface};
use draw::{get_default_context, Draw, IMAGE_SIZE};
use nalgebra::Point2;

use road::{RoadJunction, RoadSegment};

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let cr = get_default_context(&surface).unwrap();

    // draw_polylines(&cr, points);
    // cr.stroke().unwrap();
    // draw_regular_polygon(&cr, Point2::new(0.5, 0.5), 6, 0.15);
    // cr.fill().unwrap();

    let junction_1 = RoadJunction {
        pos: Point2::new(0.25, 0.25),
        segments: vec![],
    };
    let junction_2 = RoadJunction {
        pos: Point2::new(0.75, 0.75),
        segments: vec![],
    };
    let segment = RoadSegment {
        begin_junction: &junction_1,
        end_junction: &junction_2
    };
    // junction_1.segments.push(&segment);
    // junction_2.segments.push(&segment);

    segment.draw(&cr).unwrap();
    junction_1.draw(&cr).unwrap();
    junction_2.draw(&cr).unwrap();

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
