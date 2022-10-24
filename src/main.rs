extern crate cairo;
extern crate nalgebra;

mod actor;
mod draw;
mod road;

use std::fs::File;

use cairo::{Format, ImageSurface};
use draw::IMAGE_SIZE;
use nalgebra::Point2;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let mut network = road::Network::new();

    let j1 = network.register_junction(Point2::new(0.25, 0.25));
    let j2 = network.register_junction(Point2::new(0.75, 0.75));
    let s1 = network.register_segment(vec![], vec![]);

    network.link(s1, j1, j2).expect("already linked!");
    let artist = draw::Artist::new(&surface, &network);
    artist.draw_road_network().expect("explored");

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
