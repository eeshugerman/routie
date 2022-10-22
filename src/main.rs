extern crate cairo;
extern crate nalgebra;

use std::fs::File;

use cairo::{Context, Error, Format, ImageSurface};
use nalgebra::Vector2;

static IMAGE_SIZE: i32 = 600;

fn get_default_context(surface: &ImageSurface) -> Result<Context, Error> {
    let cr = Context::new(surface)?;
    cr.scale(IMAGE_SIZE as f64, IMAGE_SIZE as f64);
    cr.set_line_width(0.01);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    Ok(cr)
}

fn draw_polylines<const N: usize>(cr: &Context, points: [Vector2<f64>; N]) {
    cr.move_to(points[0].x, points[0].y);
    for point in &points[1..] {
        cr.line_to(point.x, point.y);
    }
    cr.line_to(points[0].x, points[0].y);
}

// fn draw_regular_polygon(cr: &Context, pos: Vector2<f64>, n: i8, r: f64) {
// }

fn main() {
    let points = [
        Vector2::new(0.25, 0.25),
        Vector2::new(0.25, 0.25),
        Vector2::new(0.75, 0.25),
        Vector2::new(0.25, 0.75),
        Vector2::new(0.75, 0.75),
    ];
    let more_points = [
        Vector2::new(0.35, 0.35),
        Vector2::new(0.85, 0.35),
        Vector2::new(0.35, 0.85),
        Vector2::new(0.85, 0.85),
    ];

    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let cr = get_default_context(&surface).unwrap();

    draw_polylines(&cr, points);
    cr.stroke().unwrap();

    draw_polylines(&cr, more_points);
    cr.stroke().unwrap();

    let mut file = File::create("file1.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
