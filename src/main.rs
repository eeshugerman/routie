extern crate cairo;
extern crate nalgebra;

mod error;
mod actor;
mod draw;
mod road;


use std::fs::File;

extern crate log;

use cairo::{Format, ImageSurface};
use draw::IMAGE_SIZE;
use nalgebra::Point2;

fn main() {
    env_logger::init();
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let mut network = road::Network::new();

    let j1 = network.add_junction(Point2::new(0.25, 0.25));
    let j2 = network.add_junction(Point2::new(0.25, 0.75));
    let j3 = network.add_junction(Point2::new(0.75, 0.25));
    let j4 = network.add_junction(Point2::new(0.75, 0.75));

    network.add_segment(j1, j2);
    network.add_segment(j3, j4);

    network.add_segment(j1, j3);
    network.add_segment(j2, j4);

    let artist = draw::Artist::new(&surface, &network);
    artist.draw_road_network().expect("failed to draw road network");

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
