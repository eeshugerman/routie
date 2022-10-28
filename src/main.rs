extern crate cairo;
extern crate nalgebra;

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

    let j1 = network.register_junction(Point2::new(0.25, 0.25));
    let j2 = network.register_junction(Point2::new(0.25, 0.75));
    let j3 = network.register_junction(Point2::new(0.75, 0.25));
    let j4 = network.register_junction(Point2::new(0.75, 0.75));

    let s1 = network.register_segment(vec![], vec![]);
    let s2 = network.register_segment(vec![], vec![]);
    let s3 = network.register_segment(vec![], vec![]);
    let s4 = network.register_segment(vec![], vec![]);

    network.link(s1, j1, j2).unwrap();
    network.link(s2, j3, j4).unwrap();

    network.link(s3, j1, j3).unwrap();
    network.link(s4, j2, j4).unwrap();

    let artist = draw::Artist::new(&surface, &network);
    artist.draw_road_network().unwrap();

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
