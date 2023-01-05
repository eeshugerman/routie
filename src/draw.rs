use std::f64::consts::FRAC_PI_2;
use std::f64::consts::{FRAC_PI_4, PI};

use cairo::{Context, ImageSurface};
use lyon_geom::CubicBezierSegment;
use nalgebra::{Point2, Rotation2, Vector2};

use crate::constants;
use crate::spatial::{LineLike, PointLike};
use crate::{actor, road};

pub const IMAGE_SIZE: i32 = 600;
const I_HAT: Vector2<f64> = Vector2::new(1.0, 0.0);

fn draw_regular_polygon(cairo_ctx: &cairo::Context, pos: Point2<f64>, n: i8, r: f64, theta_0: f64) {
    let start = pos + r * Rotation2::new(theta_0).matrix() * I_HAT;
    cairo_ctx.move_to(start.x, start.y);
    for x in 1..n {
        let theta = theta_0 + (x as f64 / n as f64) * 2.0 * PI;
        let point = pos + r * Rotation2::new(theta).matrix() * I_HAT;
        cairo_ctx.line_to(point.x, point.y);
    }
    cairo_ctx.line_to(start.x, start.y);
}

fn draw_road_junction_lane(cairo_ctx: &cairo::Context, lane_ctx: &road::JunctionLaneContext) {
    let (red, green, blue) = constants::ROAD_LANE_COLOR;
    cairo_ctx.set_source_rgb(red, green, blue);
    cairo_ctx.set_line_width(constants::ROAD_LANE_WIDTH_VISUAL);
    let CubicBezierSegment { from, ctrl1, ctrl2, to } = lane_ctx.get_curve().to_cubic();
    cairo_ctx.move_to(from.x, from.y);
    cairo_ctx.curve_to(ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y);
    cairo_ctx.stroke().unwrap();
}

fn draw_road_junction(cairo_ctx: &cairo::Context, junction_ctx: &road::JunctionContext) {
    let (red, green, blue) = constants::ROAD_JUNCTION_COLOR;
    cairo_ctx.set_source_rgb(red, green, blue);
    cairo_ctx.set_line_width(constants::FILLED_SHAPE_BORDER_WIDTH);
    draw_regular_polygon(
        cairo_ctx,
        junction_ctx.junction.pos,
        4,
        constants::ROAD_JUNCTION_RADIUS * f64::sqrt(2.0),
        FRAC_PI_4,
    );
    cairo_ctx.stroke().unwrap();

    for (id, lane) in junction_ctx.junction.enumerate_lanes() {
        draw_road_junction_lane(cairo_ctx, &road::JunctionLaneContext::new(junction_ctx, id, lane))
    }
}

fn draw_actor(cairo_ctx: &cairo::Context, actor_ctx: &actor::ActorContext) {
    let (red, green, blue) = constants::ACTOR_COLOR;
    cairo_ctx.set_source_rgb(red, green, blue);
    cairo_ctx.set_line_width(constants::FILLED_SHAPE_BORDER_WIDTH);

    let actor_pos = actor_ctx.get_pos();
    cairo_ctx.arc(actor_pos.x, actor_pos.y, constants::ACTOR_RADIUS_VISUAL, 0.0, 2.0 * PI);
    cairo_ctx.fill().unwrap();
}

fn draw_road_segment_lane(cairo_ctx: &cairo::Context, lane_ctx: &road::SegmentLaneContext) {
    let (red, green, blue) = constants::ROAD_LANE_COLOR;
    cairo_ctx.set_source_rgb(red, green, blue);

    cairo_ctx.set_line_width(constants::ROAD_LANE_WIDTH_VISUAL);
    let (begin_pos, end_pos) = lane_ctx.get_pos();
    cairo_ctx.move_to(begin_pos.x, begin_pos.y);
    cairo_ctx.line_to(end_pos.x, end_pos.y);
    cairo_ctx.stroke().unwrap();

    cairo_ctx.set_line_width(constants::FILLED_SHAPE_BORDER_WIDTH);
    let arrow_vec = lane_ctx.get_v(); // can't figure out how to destructure this
    let arrow_theta = FRAC_PI_2 - arrow_vec.x.atan2(arrow_vec.y);
    let arrow_size = constants::ROAD_LANE_ARROW_SIZE;
    draw_regular_polygon(cairo_ctx, lane_ctx.get_midpoint(), 3, arrow_size, arrow_theta);
    cairo_ctx.fill().unwrap();

    for (pos_param, actor) in lane_ctx.lane.actors.enumerate() {
        let actor_ctx =
            actor::ActorContext::OnRoadSegment { pos_param: *pos_param, lane_ctx, actor };
        draw_actor(cairo_ctx, &actor_ctx);
    }
}

fn draw_road_segment(cairo_ctx: &cairo::Context, segment_ctx: &road::SegmentContext) {
    let (red, green, blue) = constants::ROAD_SEGMENT_COLOR;
    cairo_ctx.set_source_rgb(red, green, blue);
    cairo_ctx.set_line_width(segment_ctx.get_width());

    let (begin_pos, end_pos) = segment_ctx.get_pos();
    cairo_ctx.move_to(begin_pos.x, begin_pos.y);
    cairo_ctx.line_to(end_pos.x, end_pos.y);
    cairo_ctx.stroke().unwrap();

    for (pos_param, actor) in segment_ctx.segment.forward_actors.enumerate() {
        let actor_ctx = actor::ActorContext::OffRoad {
            pos_param: *pos_param,
            segment_ctx,
            segment_side: road::Direction::Forward,
            actor,
        };
        draw_actor(cairo_ctx, &actor_ctx);
    }
    for (pos_param, actor) in segment_ctx.segment.backward_actors.enumerate() {
        let actor_ctx = actor::ActorContext::OffRoad {
            pos_param: *pos_param,
            segment_ctx,
            segment_side: road::Direction::Backward,
            actor,
        };
        draw_actor(cairo_ctx, &actor_ctx);
    }

    for (rank, lane) in segment_ctx.segment.forward_lanes.enumerate() {
        draw_road_segment_lane(
            cairo_ctx,
            &road::SegmentLaneContext::new(segment_ctx, road::Direction::Forward, rank, lane),
        );
    }
    for (rank, lane) in segment_ctx.segment.backward_lanes.enumerate() {
        draw_road_segment_lane(
            cairo_ctx,
            &road::SegmentLaneContext::new(segment_ctx, road::Direction::Backward, rank, lane),
        );
    }
}

pub fn draw(surface: &ImageSurface, road_network: &road::Network) {
    let cairo_ctx = &Context::new(surface).expect("Failed to create Cairo context");
    cairo_ctx.scale(IMAGE_SIZE as f64, IMAGE_SIZE as f64);
    cairo_ctx.set_line_width(0.01);
    cairo_ctx.set_source_rgb(0.0, 0.0, 0.0);

    for (id, segment) in road_network.segments.enumerate() {
        draw_road_segment(cairo_ctx, &road::SegmentContext::new(road_network, id, segment));
    }
    for (id, junction) in road_network.junctions.enumerate() {
        draw_road_junction(cairo_ctx, &road::JunctionContext::new(road_network, id, junction));
    }
}
