extern crate cairo;
extern crate nalgebra;

mod constants;
mod error;
mod actor;
mod draw;
mod road;
mod spatial;


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

    let s1 = network.add_segment(j1, j2);
    let l1 = s1.add_lane(road::Direction::Backward);
    let l2 = s1.add_lane(road::Direction::Forward);

    let s2 = network.add_segment(j3, j4);
    let l3 = s2.add_lane(road::Direction::Backward);
    let l4 = s2.add_lane(road::Direction::Forward);

    let s3 = network.add_segment(j1, j3);
    let l5 = s3.add_lane(road::Direction::Backward);
    let l6 = s3.add_lane(road::Direction::Forward);

    let s4 = network.add_segment(j2, j4);


    let artist = draw::Artist::new(&surface, &network);
    artist.draw_road_network().unwrap();

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
