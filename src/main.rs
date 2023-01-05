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

use actor::Agendum;
use cairo::{Format, ImageSurface};
use draw::IMAGE_SIZE;
use nalgebra::Point2;
use simulate::advance;

fn main() {
    env_logger::init();
    let mut network = road::Network::new();

    let j1 = network.add_junction(Point2::new(0.25, 0.25));
    let j2 = network.add_junction(Point2::new(0.25, 0.75));
    let j3 = network.add_junction(Point2::new(0.75, 0.25));
    let j4 = network.add_junction(Point2::new(0.75, 0.75));

    let (s1_id, s1) = network.add_segment(j1, j2);
    s1.add_lane(road::Direction::Backward);
    s1.add_lane(road::Direction::Forward);
    s1.add_actor(0.25, road::Direction::Backward, vec![
        Agendum::TravelTo(actor::LocationOffRoad {
            segment_id: s1_id,
            segment_side: road::Direction::Backward,
            pos_param: 0.75
        })
    ]);



    let (_, s2) = network.add_segment(j3, j4);
    s2.add_lane(road::Direction::Backward);
    s2.add_lane(road::Direction::Forward);

    let (_, s3) = network.add_segment(j1, j3);
    s3.add_lane(road::Direction::Backward);
    s3.add_lane(road::Direction::Forward);

    let _s4 = network.add_segment(j2, j4);
    // let l7 = s4.add_lane(road::Direction::Backward);
    // let l8 = s4.add_lane(road::Direction::Forward);

    network.connect_junctions();


    for result in std::fs::read_dir("./frames/").unwrap() {
        match result {
            Ok(entry) => if entry.file_name() != ".gitkeep" {
                std::fs::remove_file(entry.path()).unwrap()
            },
            Err(_) => (),
        }
    }

    for i in 0..15 {
        // TODO: redraw actors only
        let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
        let mut file = File::create(format!("./frames/{}.png", i)).unwrap();
        draw::draw(&surface, &network);
        surface.write_to_png(&mut file).unwrap();
        network = advance(network);
    }

}
