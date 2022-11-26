use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, ImageSurface};
use nalgebra::{Point2, Rotation2, Vector2};

use crate::constants::ROAD_JUNCTION_COLOR;
use crate::constants::ROAD_JUNCTION_RADIUS;
use crate::constants::ROAD_LANE_COLOR;
use crate::constants::ROAD_LANE_WIDTH;
use crate::constants::ROAD_SEGMENT_COLOR;
use crate::constants::ROAD_SEGMENT_WIDTH;
use crate::error::GenericError;
use crate::road;
use crate::spatial::{located, LineLike};

pub const IMAGE_SIZE: i32 = 600;
const I_HAT: Vector2<f64> = Vector2::new(1.0, 0.0);

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

    fn draw_road_segment_lane(
        &self,
        located_lane: &located::SegmentLane,
    ) -> Result<(), GenericError> {
        let (red, green, blue) = ROAD_LANE_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(ROAD_LANE_WIDTH);

        let (begin_pos, end_pos) = located_lane.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke()?;
        Ok(())
    }

    fn draw_road_segment(&self, located_segment: &located::Segment) -> Result<(), GenericError> {
        let (red, green, blue) = ROAD_SEGMENT_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(ROAD_SEGMENT_WIDTH);

        let (begin_pos, end_pos) = located_segment.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke()?;

        let located::Segment(_, segment) = located_segment;
        for lane in &segment.forward_lanes {
            self.draw_road_segment_lane(&located::SegmentLane(located_segment, &lane))?
        }
        for lane in &segment.backward_lanes {
            self.draw_road_segment_lane(&located::SegmentLane(located_segment, &lane))?
        }
        Ok(())
    }

    pub fn draw_road_network(&self) -> Result<(), GenericError> {
        for segment in self.road_network.get_segments() {
            self.draw_road_segment(&located::Segment(self.road_network, segment))?;
        }
        for junction in self.road_network.get_junctions() {
            self.draw_road_junction(junction)?;
        }
        // self.road_network.get_junctions().for_each(self.draw_road_junction)
        Ok(())
    }
}
