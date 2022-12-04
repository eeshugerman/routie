use std::f64::consts::FRAC_PI_2;
use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, ImageSurface};
use nalgebra::{Point2, Rotation2, Vector2};

use crate::constants::{
    FILLED_SHAPE_BORDER_WIDTH, ROAD_JUNCTION_COLOR, ROAD_JUNCTION_RADIUS, ROAD_LANE_ARROW_SIZE,
    ROAD_LANE_COLOR, ROAD_LANE_WIDTH_VISUAL, ROAD_SEGMENT_COLOR,
};
use crate::road;
use crate::spatial::LineLike;

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

    fn draw_road_junction_lane_segment(&self, lane_ctx: &road::JunctionLaneContext) {
        let (red, green, blue) = ROAD_LANE_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(ROAD_LANE_WIDTH_VISUAL);
        let (begin_pos, end_pos) = lane_ctx.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke().unwrap();
    }

    fn draw_road_junction(&self, junction_ctx: &road::JunctionContext) {
        let (red, green, blue) = ROAD_JUNCTION_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(FILLED_SHAPE_BORDER_WIDTH);
        // TODO: use draw_polylines 
        self.draw_regular_polygon(junction_ctx.itself.pos, 4, ROAD_JUNCTION_RADIUS * f64::sqrt(2.0), FRAC_PI_4);
        self.cairo_ctx.stroke().unwrap();

        // for (id, lane) in junction_ctx.itself.lanes.enumerate() {
        //     self.draw_road_junction_lane(&road::JunctionLaneContext::new(junction_ctx, id, lane))
        // }
    }

    fn draw_road_segment_lane(&self, lane_ctx: &road::SegmentLaneContext) {
        let (red, green, blue) = ROAD_LANE_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);

        self.cairo_ctx.set_line_width(ROAD_LANE_WIDTH_VISUAL);
        let (begin_pos, end_pos) = lane_ctx.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke().unwrap();

        self.cairo_ctx.set_line_width(FILLED_SHAPE_BORDER_WIDTH);
        let vec = lane_ctx.get_v(); // can't figure out how to destructure this
        let theta = FRAC_PI_2 - vec.x.atan2(vec.y);
        self.draw_regular_polygon(lane_ctx.get_midpoint(), 3, ROAD_LANE_ARROW_SIZE, theta);
        self.cairo_ctx.fill().unwrap();
    }

    fn draw_road_segment(&self, segment_ctx: &road::SegmentContext) {
        let (red, green, blue) = ROAD_SEGMENT_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(segment_ctx.get_width());

        let (begin_pos, end_pos) = segment_ctx.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke().unwrap();

        for (rank, lane) in segment_ctx.itself.forward_lanes.enumerate() {
            self.draw_road_segment_lane(&road::SegmentLaneContext::new(
                segment_ctx,
                road::Direction::Forward,
                rank,
                lane,
            ));
        }
        for (rank, lane) in segment_ctx.itself.backward_lanes.enumerate() {
            self.draw_road_segment_lane(&road::SegmentLaneContext::new(
                segment_ctx,
                road::Direction::Backward,
                rank,
                lane,
            ));
        }
    }

    pub fn draw_road_network(&self) {
        for (id, segment) in self.road_network.get_segments().enumerate() {
            self.draw_road_segment(&road::SegmentContext::new(self.road_network, id, segment));
        }
        for (id, junction) in self.road_network.get_junctions().enumerate() {
            self.draw_road_junction(&road::JunctionContext::new(self.road_network, id, junction));
        }
    }
}
