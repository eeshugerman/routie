extern crate nalgebra;

use nalgebra::Point2;

use crate::road::{Direction, JunctionLaneContext, PosParam, SegmentContext, SegmentLaneContext};

#[allow(dead_code)]
pub struct Actor {
    // max_speed: f64,
    // route: Option<Vec<RouteStep>>,
    // agenda: Vec<AgendaStep>
}

impl Actor {
    pub fn new() -> Self { Self {} }
}

pub enum ActorLaneContext<'a> {
    Segment(SegmentLaneContext<'a>),
    Junction(JunctionLaneContext<'a>),
}

pub enum ActorContext<'a> {
    OnRoad { pos_param: PosParam, lane_ctx: ActorLaneContext<'a> },
    OffRoad { pos_param: PosParam, segment_ctx: SegmentContext<'a>, segment_side: Direction },
}
