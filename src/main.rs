extern crate cairo;

use cairo::{Context, Error, Format, ImageSurface};
use std::fs::File;

struct Pos {
    x: f64,
    y: f64,
}

fn draw_polygon<const N: usize>(points: [Pos; N]) -> Result<ImageSurface, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)?;
    let cr = Context::new(&surface)?;
    cr.scale(600.0, 600.0);
    cr.set_line_width(0.01);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.move_to(points[0].x, points[0].y);
    for point in &points[1..] {
        cr.line_to(point.x, point.y);
    }
    cr.line_to(points[0].x, points[0].y);
    cr.stroke()?;
    return Ok(surface);
}

fn main() {
    let points = [
        Pos { x: 0.25, y: 0.25 },
        Pos { x: 0.75, y: 0.25 },
        Pos { x: 0.75, y: 0.75 },
    ];
    let surface = draw_polygon(points).expect("Failed to draw image");
    let mut file = File::create("file.png").expect("Can't create file");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}
