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

#[allow(dead_code)]
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

impl<'a> ActorContext<'a> {
    pub fn advance(&mut self) {
        match self {
            ActorContext::OnRoadSegment { pos_param, lane_ctx, actor } => {
                lane_ctx.lane.actors.remove(*pos_param);
                lane_ctx.lane.actors.insert(*pos_param + 0.1, **actor);
            },
            ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => todo!(),
            ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => todo!(),
        }
    }
}
