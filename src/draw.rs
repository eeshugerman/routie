use std::f64::consts::FRAC_PI_2;
use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, ImageSurface};
use lyon_geom::CubicBezierSegment;
use nalgebra::{Point2, Rotation2, Vector2};

use crate::actor::ActorContext;
use crate::constants;
use crate::spatial::LineLike;
use crate::{actor, road};

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
        Artist { road_network, cairo_ctx }
    }

    // fn draw_polylines<const N: usize>(&self, points: [Point2<f64>; N]) {
    //     self.cairo_ctx.move_to(points[0].x, points[0].y);
    //     for point in &points[1..] {
    //         self.cairo_ctx.line_to(point.x, point.y);
    //     }
    //     self.cairo_ctx.line_to(points[0].x, points[0].y);
    // }

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

    fn draw_road_junction_lane(&self, lane_ctx: &road::JunctionLaneContext) {
        let (red, green, blue) = constants::ROAD_LANE_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(constants::ROAD_LANE_WIDTH_VISUAL);
        let CubicBezierSegment { from, ctrl1, ctrl2, to } = lane_ctx.get_curve().to_cubic();
        self.cairo_ctx.move_to(from.x, from.y);
        self.cairo_ctx.curve_to(ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y);
        self.cairo_ctx.stroke().unwrap();
    }

    fn draw_road_junction(&self, junction_ctx: &road::JunctionContext) {
        let (red, green, blue) = constants::ROAD_JUNCTION_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(constants::FILLED_SHAPE_BORDER_WIDTH);
        self.draw_regular_polygon(
            junction_ctx.junction.pos,
            4,
            constants::ROAD_JUNCTION_RADIUS * f64::sqrt(2.0),
            FRAC_PI_4,
        );
        self.cairo_ctx.stroke().unwrap();

        for (id, lane) in junction_ctx.junction.enumerate_lanes() {
            self.draw_road_junction_lane(&road::JunctionLaneContext::new(junction_ctx, id, lane))
        }
    }

    fn draw_actor(&self, actor_ctx: &actor::ActorContext) {
        let (red, green, blue) = constants::ACTOR_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(constants::FILLED_SHAPE_BORDER_WIDTH);

        let actor_pos = match actor_ctx {
            ActorContext::OnRoadSegment { pos_param, lane_ctx, actor } => {
                let (lane_begin_pos, _) = lane_ctx.get_pos();
                lane_begin_pos + *pos_param * lane_ctx.get_v()
            }
            ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => {
                todo!()
            }
            ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => {
                todo!()
            }
        };

        self.cairo_ctx.arc(actor_pos.x, actor_pos.y, constants::ACTOR_RADIUS_VISUAL, 0.0, 2.0 * PI);
        self.cairo_ctx.fill().unwrap();
    }

    fn draw_road_segment_lane(&self, lane_ctx: &road::SegmentLaneContext) {
        let (red, green, blue) = constants::ROAD_LANE_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);

        self.cairo_ctx.set_line_width(constants::ROAD_LANE_WIDTH_VISUAL);
        let (begin_pos, end_pos) = lane_ctx.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke().unwrap();

        self.cairo_ctx.set_line_width(constants::FILLED_SHAPE_BORDER_WIDTH);
        let arrow_vec = lane_ctx.get_v(); // can't figure out how to destructure this
        let arrow_theta = FRAC_PI_2 - arrow_vec.x.atan2(arrow_vec.y);
        let arrow_size = constants::ROAD_LANE_ARROW_SIZE;
        self.draw_regular_polygon(lane_ctx.get_midpoint(), 3, arrow_size, arrow_theta);
        self.cairo_ctx.fill().unwrap();

        for (pos_param, actor) in lane_ctx.lane.actors.enumerate() {
            let actor_ctx = ActorContext::OnRoadSegment { pos_param: *pos_param, lane_ctx, actor };
            self.draw_actor(&actor_ctx);
        }
    }

    fn draw_road_segment(&self, segment_ctx: &road::SegmentContext) {
        let (red, green, blue) = constants::ROAD_SEGMENT_COLOR;
        self.cairo_ctx.set_source_rgb(red, green, blue);
        self.cairo_ctx.set_line_width(segment_ctx.get_width());

        let (begin_pos, end_pos) = segment_ctx.get_pos();
        self.cairo_ctx.move_to(begin_pos.x, begin_pos.y);
        self.cairo_ctx.line_to(end_pos.x, end_pos.y);
        self.cairo_ctx.stroke().unwrap();

        for (rank, lane) in segment_ctx.segment.forward_lanes.enumerate() {
            self.draw_road_segment_lane(&road::SegmentLaneContext::new(
                segment_ctx,
                road::Direction::Forward,
                rank,
                lane,
            ));
        }
        for (rank, lane) in segment_ctx.segment.backward_lanes.enumerate() {
            self.draw_road_segment_lane(&road::SegmentLaneContext::new(
                segment_ctx,
                road::Direction::Backward,
                rank,
                lane,
            ));
        }
    }

    pub fn draw_road_network(&self) {
        for (id, segment) in self.road_network.segments.enumerate() {
            self.draw_road_segment(&road::SegmentContext::new(self.road_network, id, segment));
        }
        for (id, junction) in self.road_network.junctions.enumerate() {
            self.draw_road_junction(&road::JunctionContext::new(self.road_network, id, junction));
        }
    }
}
