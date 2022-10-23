use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, Error, ImageSurface};
use nalgebra::{Point2, Rotation2, Vector2};

use crate::road::{RoadJunction, RoadSegment};

pub const IMAGE_SIZE: i32 = 600;
const I_HAT: Vector2<f64> = Vector2::new(1.0, 0.0);

const ROAD_JUNCTION_RADIUS: f64 = 0.05;
const ROAD_JUNCTION_COLOR: (f64, f64, f64) = (0.7, 0.7, 0.7);
const ROAD_SEGMENT_WIDTH: f64 = 0.05;
const ROAD_SEGMENT_COLOR: (f64, f64, f64) = (1.0, 1.0, 1.0);

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

fn draw_regular_polygon(cr: &Context, pos: Point2<f64>, n: i8, r: f64, theta_0: f64) {
    let start = pos + r * Rotation2::new(theta_0).matrix() * I_HAT;
    cr.move_to(start.x, start.y);
    for x in 1..n {
        let theta = theta_0 + (x as f64 / n as f64) * 2.0 * PI;
        let point = pos + r * Rotation2::new(theta).matrix() * I_HAT;
        cr.line_to(point.x, point.y);
    }
    cr.line_to(start.x, start.y);
}

pub trait Draw {
    fn draw(&self, cr: &Context) -> Result<(), Error>;
}

impl Draw for RoadJunction<'_> {
    fn draw(&self, cr: &Context) -> Result<(), Error> {
        let (red, green, blue) = ROAD_JUNCTION_COLOR;
        cr.set_source_rgb(red, green, blue);
        draw_regular_polygon(cr, self.pos, 4, ROAD_JUNCTION_RADIUS, FRAC_PI_4);
        cr.fill()
    }
}

impl Draw for RoadSegment<'_> {
    fn draw(&self, cr: &Context) -> Result<(), Error> {
        let (red, green, blue) = ROAD_SEGMENT_COLOR;
        cr.set_source_rgb(red, green, blue);
        cr.set_line_width(ROAD_SEGMENT_WIDTH);
        cr.move_to(self.begin_junction.pos.x, self.begin_junction.pos.y);
        cr.line_to(self.end_junction.pos.x, self.end_junction.pos.y);
        cr.stroke()
    }
}
