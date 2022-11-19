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

pub struct Artist<'a> {
    road_network: &'a road::Network,
    cairo_ctx: Context,
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

    fn draw_polylines<const N: usize>(&self, points: [Point2<f64>; N]) {
        self.cairo_ctx.move_to(points[0].x, points[0].y);
        for point in &points[1..] {
            self.cairo_ctx.line_to(point.x, point.y);
        }
        self.cairo_ctx.line_to(points[0].x, points[0].y);
    }

    fn draw_regular_polygon(&self, pos: Point2<f64>, n: i8, r: f64, theta_0: f64) {
        let start = pos + r * Rotation2::new(theta_0).matrix() * I_HAT;
        self.cairo_ctx.move_to(start.x, start.y);
        for x in 1..n {
            let theta = theta_0 + (x as f64 / n as f64) * 2.0 * PI;
            let point = pos + r * Rotation2::new(theta).matrix() * I_HAT;
            self.cairo_ctx.line_to(point.x, point.y);
        }
        self.cairo_ctx.line_to(start.x, start.y);
    }

    fn draw_road_junction(&self, junction: &road::Junction) -> Result<(), GenericError> {
        let (red, green, blue) = ROAD_JUNCTION_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.draw_regular_polygon(junction.pos, 4, ROAD_JUNCTION_RADIUS, FRAC_PI_4);
        self.cairo_ctx.fill()?;
        Ok(())
    }

    fn draw_road_segment(&self, segment: &road::Segment) -> Result<(), GenericError> {
        let (red, green, blue) = ROAD_SEGMENT_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(ROAD_SEGMENT_WIDTH);
        let (begin_junction, end_junction) = self.road_network
            .get_segment_junctions(segment)
            .map_err(|_| RoutieError::UnlinkedSegment)?;
        self.cairo_ctx.move_to(begin_junction.pos.x, begin_junction.pos.y);
        self.cairo_ctx.line_to(end_junction.pos.x, end_junction.pos.y);
        self.cairo_ctx.stroke()?;
        Ok(())
    }

    pub fn draw_road_network(&self) -> Result<(), GenericError> {
        for segment in self.road_network.get_segments() {
            self.draw_road_segment(segment)?;
        }
        for junction in self.road_network.get_junctions() {
            self.draw_road_junction(junction)?;
        }
        // self.road_network.get_junctions().for_each(self.draw_road_junction)
        Ok(())
    }
}
