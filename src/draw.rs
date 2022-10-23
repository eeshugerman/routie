use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, Error, ImageSurface};
use nalgebra::{Point2, Rotation2, Vector2};

use crate::road::RoadJunction;

pub static IMAGE_SIZE: i32 = 600;
static I_HAT: Vector2<f64> = Vector2::new(1.0, 0.0);
static J_HAT: Vector2<f64> = Vector2::new(0.0, 1.0);

pub fn get_default_context(surface: &ImageSurface) -> Result<Context, Error> {
    let cr = Context::new(surface)?;
    cr.scale(IMAGE_SIZE as f64, IMAGE_SIZE as f64);
    cr.set_line_width(0.01);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    Ok(cr)
}

fn draw_polylines<const N: usize>(cr: &Context, points: [Point2<f64>; N]) {
    cr.move_to(points[0].x, points[0].y);
    for point in &points[1..] {
        cr.line_to(point.x, point.y);
    }
    cr.line_to(points[0].x, points[0].y);
}

fn draw_regular_polygon(cr: &Context, pos: Point2<f64>, n: i8, r: f64, delta: f64) {
    let rot = Rotation2::new(delta);
    let start = pos + r * rot.matrix() * I_HAT;
    cr.move_to(start.x, start.y);
    for x in 1..n {
        let rot = Rotation2::new(delta + (x as f64 / n as f64) * 2.0 * PI);
        let point = pos + r * rot.matrix() * I_HAT;
        cr.line_to(point.x, point.y);
    }
    cr.line_to(start.x, start.y);
}

pub trait Draw {
    fn draw(&self, cr: &Context) -> Result<(), Error>;
}
const ROAD_JUNCTION_RADIUS: f64 = 0.05;
const ROAD_JUNCTION_COLOR: (f64, f64, f64) = (0.4, 0.4, 0.4);

impl Draw for RoadJunction {
    fn draw(&self, cr: &Context) -> Result<(), Error> {
        let (r, g, b) = ROAD_JUNCTION_COLOR;
        cr.set_source_rgb(r, g, b);
        draw_regular_polygon(cr, self.pos, 4, ROAD_JUNCTION_RADIUS, FRAC_PI_4);
        cr.fill()
    }
}
