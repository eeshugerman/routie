extern crate cairo;
extern crate nalgebra;

#[macro_use]
mod util;
mod actor;
mod constants;
mod draw;
mod error;
mod road;
mod simulate;
mod spatial;

extern crate log;

use std::io::Write;

use actor::Agendum;
use draw::IMAGE_SIZE;
use nalgebra::Point2;

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
    s1.add_actor(
        0.25,
        road::Direction::Backward,
        vec![Agendum::TravelTo(actor::LocationOffRoad {
            segment_id: s1_id,
            segment_side: road::Direction::Backward,
            pos_param: 0.75,
        })],
    );

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

    // build movie
    // based on https://gist.github.com/tetsu-koba/14083c6705b69017bbc7fb97602f610a
    let _ = std::fs::remove_file("./out.mp4");
    let mut ffmpeg = std::process::Command::new("/bin/sh")
        .args(&["-c", &format!(
            "ffmpeg -r {framerate} -f rawvideo -pix_fmt bgra -s {width}x{height} -i pipe: -pix_fmt yuv420p -r {framerate} -y out.mp4",
            width = draw::IMAGE_SIZE,
            height = draw::IMAGE_SIZE,
            framerate = 1
        )])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let mut frames: Vec<cairo::ImageSurfaceDataOwned> = Vec::new();
    for _i in 0..15 {
        println!("{}", _i);
        // TODO: redraw actors only
        let surface =
            cairo::ImageSurface::create(cairo::Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
        draw::draw(&surface, &network);
        frames.push(surface.take_data().unwrap());
        network = simulate::advance(network);
    }
    let ffmpeg_stdin = ffmpeg.stdin.as_mut().unwrap();
    for frame in frames {
        ffmpeg_stdin.write_all(&frame).unwrap();
    }
    ffmpeg.wait().expect("child process wasn't running");
}
