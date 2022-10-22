extern crate cairo;

use cairo::{Context, Error, Format, ImageSurface};
use std::fs::File;

static IMAGE_SIZE: i32 = 600;

struct Pos {
    x: f64,
    y: f64,
}

fn draw_polylines<const N: usize>(surface: &ImageSurface, points: [Pos; N], fill: bool) -> Result<(), Error> {
    let cr = Context::new(surface).expect("Failed to create surface");
    cr.scale(IMAGE_SIZE as f64, IMAGE_SIZE as f64);
    cr.set_line_width(0.01);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.move_to(points[0].x, points[0].y);
    for point in &points[1..] {
        cr.line_to(point.x, point.y);
    }
    cr.line_to(points[0].x, points[0].y);
    if fill {
        cr.fill()
    } else {
        cr.stroke()
    }
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).expect("Failed to create surface");
    let points = [
        Pos { x: 0.25, y: 0.25 },
        Pos { x: 0.75, y: 0.25 },
        Pos { x: 0.25, y: 0.75 },
        Pos { x: 0.75, y: 0.75 },
    ];
    let more_points = [
        Pos { x: 0.35, y: 0.35 },
        Pos { x: 0.85, y: 0.35 },
        Pos { x: 0.35, y: 0.85 },
        Pos { x: 0.85, y: 0.85 },
    ];
    draw_polylines(&surface, points, false).expect("Failed to draw image");
    draw_polylines(&surface, more_points, false).expect("Failed to draw image");
    let mut file = File::create("file1.png").expect("Can't create file");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(e) => println!("Error create file.png: {}", e),
    }
}
