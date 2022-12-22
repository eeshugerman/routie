extern crate cairo;
extern crate nalgebra;

#[macro_use]
mod util;
mod actor;
mod constants;
mod draw;
mod error;
mod road;
mod spatial;
mod simulate;

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
    l1.add_actor(0.5);
    let _l2 = s1.add_lane(road::Direction::Forward);


    let s2 = network.add_segment(j3, j4);
    let _l3 = s2.add_lane(road::Direction::Backward);
    let _l4 = s2.add_lane(road::Direction::Forward);

    let s3 = network.add_segment(j1, j3);
    let _l5 = s3.add_lane(road::Direction::Backward);
    let _l6 = s3.add_lane(road::Direction::Forward);

    let _s4 = network.add_segment(j2, j4);
    // let l7 = s4.add_lane(road::Direction::Backward);
    // let l8 = s4.add_lane(road::Direction::Forward);

    network.connect_junctions();

    let artist = draw::Artist::new(&surface, &network);

    for i in 0..5 {
        artist.draw_road_network();
        let mut file = File::create(format!("frames/{}.png", i)).unwrap();
        surface.write_to_png(&mut file).unwrap();
    }

}
