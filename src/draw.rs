use std::f64::consts::PI;

use cairo::{ImageSurface, Context, Error};
use nalgebra::{Vector2, Point2, Rotation2};

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

fn draw_regular_polygon(cr: &Context, pos: Point2<f64>, n: i8, r: f64) {
    let start = pos + r * I_HAT;
    cr.move_to(start.x, start.y);
    for x in 1..n {
        let rot = Rotation2::new((x as f64 / n as f64) * 2.0 * PI);
        let point = pos + r * rot.matrix() * I_HAT;
        cr.line_to(point.x, point.y);
    }
    cr.line_to(start.x, start.y);
}

pub trait Draw {
    fn draw(&self, cr: &Context);
}
const ROAD_JUNCTION_RADIUS: f64 = 0.05;

impl Draw for RoadJunction {
    fn draw(&self, cr: &Context) {
        draw_regular_polygon(cr, self.pos, 4, ROAD_JUNCTION_RADIUS)
    }
}
