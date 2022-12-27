extern crate nalgebra;

use crate::road::{Direction, JunctionLaneContext, PosParam, SegmentContext, SegmentLaneContext};

pub struct Actor {
    // max_speed: f64,
    // route: Option<Vec<RouteStep>>,
    // agenda: Vec<AgendaStep>
}

impl Actor {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum ActorContext<'a> {
    OnRoadSegment {
        pos_param: PosParam,
        lane_ctx: &'a SegmentLaneContext<'a>,
        actor: &'a Actor,
    },
    OnRoadJunction {
        pos_param: PosParam,
        lane_ctx: &'a JunctionLaneContext<'a>,
        actor: &'a Actor,
    },
    OffRoad {
        pos_param: PosParam,
        segment_ctx: SegmentContext<'a>,
        segment_side: Direction,
        actor: &'a Actor,
    },
}
