use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, ImageSurface};
use nalgebra::{Point2, Rotation2, Vector2};

use crate::road;
use crate::error::{RoutieError,CairoError,GenericError};

pub const IMAGE_SIZE: i32 = 600;
const I_HAT: Vector2<f64> = Vector2::new(1.0, 0.0);

const ROAD_JUNCTION_RADIUS: f64 = 0.05;
const ROAD_JUNCTION_COLOR: (f64, f64, f64) = (0.7, 0.7, 0.7);
const ROAD_SEGMENT_WIDTH: f64 = 0.05;
const ROAD_SEGMENT_COLOR: (f64, f64, f64) = (1.0, 1.0, 1.0);

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
    fn draw(&self, cr: &Context, network: &road::Network) -> Result<(), GenericError>;
}

pub struct Artist<'a> {
    road_network: &'a road::Network,
    cairo_ctx: Context,
}

impl Draw for road::Junction {
    fn draw(&self, cr: &Context, _network: &road::Network) -> Result<(), GenericError> {
        let (red, green, blue) = ROAD_JUNCTION_COLOR;
        cr.set_source_rgb(red, green, blue);
        draw_regular_polygon(cr, self.pos, 4, ROAD_JUNCTION_RADIUS, FRAC_PI_4);
        cr.fill()?;
        Ok(())
    }
}

impl Draw for road::Segment {
    fn draw(&self, cr: &Context, network: &road::Network) -> Result<(), GenericError> {
        let (red, green, blue) = ROAD_SEGMENT_COLOR;
        cr.set_source_rgb(red, green, blue);
        cr.set_line_width(ROAD_SEGMENT_WIDTH);
        let (begin_junction, end_junction) = network
            .get_segment_junctions(self.id)
            .map_err(|_| RoutieError::UnlinkedSegment)?;
        cr.move_to(begin_junction.pos.x, begin_junction.pos.y);
        cr.line_to(end_junction.pos.x, end_junction.pos.y);
        cr.stroke()?;
        Ok(())
    }
}

impl Artist<'_> {
    pub fn new<'a>(surface: &'a ImageSurface, road_network: &'a road::Network) -> Artist<'a> {
        let cairo_ctx = Context::new(surface).expect("Failed to create Cairo context");
        cairo_ctx.scale(IMAGE_SIZE as f64, IMAGE_SIZE as f64);
        cairo_ctx.set_line_width(0.01);
        cairo_ctx.set_source_rgb(0.0, 0.0, 0.0);
        Artist {
            road_network,
            cairo_ctx,
        }
    }

    pub fn draw_road_network(&self) -> Result<(), GenericError> {
        for (_, segment) in self.road_network.segments() {
            segment.draw(&self.cairo_ctx, self.road_network)?;
        }
        for (_, junction) in self.road_network.junctions() {
            junction.draw(&self.cairo_ctx, self.road_network)?;
        }
        Ok(())
    }
}
