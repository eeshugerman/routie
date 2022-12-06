extern crate nalgebra;

use nalgebra::Point2;

use crate::road::{Direction, JunctionLaneContext, PosParam, SegmentContext, SegmentLaneContext};

#[allow(dead_code)]
pub struct Actor {
    id: i32,
    location: Point2<f64>,
    max_speed: f64,
    // route: Option<Vec<RouteStep>>,
    // agenda: Vec<AgendaStep>
}

pub enum ActorLaneContext<'a> {
    Segment(SegmentLaneContext<'a>),
    Junction(JunctionLaneContext<'a>),
}

pub enum ActorContext<'a> {
    OnRoad { pos_param: PosParam, lane_ctx: ActorLaneContext<'a> },
    OffRoad { pos_param: PosParam, segment_ctx: SegmentContext<'a>, segment_side: Direction },
}
