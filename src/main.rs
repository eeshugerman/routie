extern crate cairo;
extern crate nalgebra;

mod actor;
mod road;
mod draw;

use std::{fs::File, f64::consts::PI};

use cairo::{ImageSurface, Format};
use draw::{IMAGE_SIZE, get_default_context, Draw};
use nalgebra::{Point2};

use road::{RoadJunction, RoadSegment};


fn main() {
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let cr = get_default_context(&surface).unwrap();

    // draw_polylines(&cr, points);
    // cr.stroke().unwrap();
    // draw_regular_polygon(&cr, Point2::new(0.5, 0.5), 6, 0.15);
    // cr.fill().unwrap();

    let junction = RoadJunction {pos: Point2::new(0.25, 0.25), segments: vec![] };
    junction.draw(&cr);

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
