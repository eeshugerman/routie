extern crate nalgebra;

use crate::{constants, road};

#[derive(Clone)]
struct AgendaStep;
#[derive(Clone)]
struct RouteStep;

#[derive(Clone)]
pub struct Actor {
    max_speed: f64,
    route: Option<Vec<RouteStep>>,
    agenda: Vec<AgendaStep>,
}

impl Actor {
    pub fn new() -> Self {
        Self { max_speed: constants::ACTOR_MAX_SPEED, route: Option::None, agenda: Vec::new() }
    }
}

pub enum ActorContext<'a> {
    OnRoadSegment {
        pos_param: road::PosParam,
        lane_ctx: &'a road::SegmentLaneContext<'a>,
        actor: &'a Actor,
    },
    OnRoadJunction {
        pos_param: road::PosParam,
        lane_ctx: &'a road::JunctionLaneContext<'a>,
        actor: &'a Actor,
    },
    OffRoad {
        pos_param: road::PosParam,
        segment_ctx: road::SegmentContext<'a>,
        segment_side: road::Direction,
        actor: &'a Actor,
    },
}

impl ActorContext<'_> {
    pub fn advance(&self, network_next: &mut road::Network) {
        match self {
            ActorContext::OnRoadSegment { pos_param, lane_ctx, actor } => {
                let lane_next = network_next
                    .segments
                    .get_mut(&lane_ctx.segment_ctx.id)
                    .unwrap()
                    .get_lanes_mut(lane_ctx.direction)
                    .get_mut(&lane_ctx.rank)
                    .unwrap();
                lane_next.actors.insert(pos_param + 0.1, (*actor).clone())
            }
            ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => todo!(),
            ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => todo!(),
        }
    }
}
