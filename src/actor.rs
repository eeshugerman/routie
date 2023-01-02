extern crate nalgebra;

use crate::{constants, road};

#[derive(Clone, Copy, Debug)]
struct Location;

#[derive(Clone, Copy, Debug)]
enum AgendaStep {
    SleepFor(i32),
    TravelTo(Location),
}

#[derive(Clone, Copy, Debug)]
pub enum RouteStep {
    ArriveAt(road::PosParam),
    LaneChange(road::SegmentLaneRank),
    TurnOnto(road::JunctionLaneId),
    ContinueOnto(road::SegmentLaneRank),
}

#[derive(Clone, Debug)]
pub struct Actor {
    max_speed: f64,
    route: Vec<RouteStep>,
    agenda: Vec<AgendaStep>,
}

#[derive(Debug)]
pub struct NullRouteError;

type AgendaStatus = Option<AgendaStep>;

impl Actor {
    pub fn new() -> Self {
        Self { max_speed: constants::ACTOR_MAX_SPEED, route: Vec::new(), agenda: Vec::new() }
    }

    pub fn route_peek(&self) -> Option<RouteStep> {
        match &self.route.as_slice() {
            [] => None,
            [_rest @ .., last] => Some(*last),
        }
    }

    pub fn route_pop(&mut self) -> Result<RouteStep, NullRouteError> {
        if self.route.len() > 0 {
            Ok(self.route.remove(self.route.len() - 1))
        } else {
            Err(NullRouteError)
        }
    }

    pub fn agenda_peek(&self) -> AgendaStatus {
        self.agenda.last().copied()
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
    pub fn advance(&self, network: &mut road::Network) {
        match self {
            ActorContext::OnRoadSegment { pos_param: pos_param_current, lane_ctx, actor } => {
                let lane_current = network
                    .segments
                    .get_mut(&lane_ctx.segment_ctx.id)
                    .unwrap()
                    .get_lanes_mut(lane_ctx.direction)
                    .get_mut(&lane_ctx.rank)
                    .unwrap();

                let pos_param_next_naive =
                    pos_param_current + actor.max_speed * constants::SIM_TIME_STEP;
                match actor.route_peek() {
                    None => {
                        // move off road
                        todo!();
                    },
                    Some(step) => match step {
                        RouteStep::ArriveAt(pos_param_target) => {
                            let mut actor_next = (*actor).clone();
                            if pos_param_next_naive >= pos_param_target {
                                actor_next.route_pop().unwrap();
                                lane_current.actors.insert(pos_param_target, actor_next)
                            } else {
                                lane_current.actors.insert(pos_param_next_naive, (*actor).clone())
                            }
                        }
                        RouteStep::LaneChange(lane_rank) => todo!(),
                        RouteStep::TurnOnto(lane_id) => {
                            if pos_param_next_naive > 1.0 {
                                todo!()
                            } else {
                                lane_current.actors.insert(pos_param_next_naive, (*actor).clone())
                            }
                        }
                        RouteStep::ContinueOnto(lane_rank) => todo!(),
                    },
                }
            }
            ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => todo!(),
            ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => todo!(),
        }
    }
}
