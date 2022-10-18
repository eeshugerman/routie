extern crate cairo;

use cairo::{Context, Format, ImageSurface, Error};
use std::fs::File;

struct Vec2 {
    x: f32,
    y: f32
}

fn draw_rectangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, p_4: Vec2) -> Result<ImageSurface, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)?;
    let cr = Context::new(&surface)?;
    cr.scale(600.0, 600.0);

    cr.set_line_width(0.01);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    // cr.rectangle(); // TODO
    cr.stroke()?;
    return Ok(surface);
}

fn draw() -> Result<ImageSurface, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)?;
    let cr = Context::new(&surface)?;
    cr.scale(600.0, 600.0);

    cr.set_line_width(0.01);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.rectangle(0.25, 0.25, 0.5, 0.5);
    cr.rectangle(0.5, 0.5, 0.5, 0.5);
    cr.stroke()?;
    return Ok(surface);
}

fn main() {
    let surface = draw().expect("Failed to draw image");
    let mut file = File::create("file.png").expect("Can't create file");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}
