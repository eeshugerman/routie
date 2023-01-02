extern crate nalgebra;

use crate::{constants, error::RoutieError, road, util::seq_indexed_store::SeqIndexedStore};

#[derive(Clone, Copy, Debug)]
pub struct LocationOnRoad {
    segment_id: road::SegmentId,
    lane_direction: road::Direction,
    lane_rank: road::SegmentLaneRank,
    pos_param: road::PosParam,
}

#[derive(Clone, Copy, Debug)]
pub struct LocationOffRoad {
    segment_id: road::SegmentId,
    segment_side: road::Direction,
    pos_param: road::PosParam,
}

#[derive(Clone, Copy, Debug)]
pub enum Agendum {
    SleepFor(i32),
    TravelTo(LocationOffRoad),
}

#[derive(Clone, Copy, Debug)]
pub enum RouteStep {
    ArriveAt(road::PosParam),
    LaneChange(road::SegmentLaneRank),
    TurnOnto(road::JunctionLaneId),
}

#[derive(Clone, Debug)]
pub struct Actor {
    max_speed: f64,
    route: Vec<RouteStep>,
    agenda: Vec<Agendum>,
}

#[derive(Debug)]
pub struct NullRouteError;
#[derive(Debug)]
struct NoSuchLocationError;

type AgendaStatus = Option<Agendum>;

impl Actor {
    pub fn new() -> Self {
        Self { max_speed: constants::ACTOR_MAX_SPEED, route: Vec::new(), agenda: Vec::new() }
    }

    pub fn route_push(&mut self, item: RouteStep) {
        self.route.push(item)
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

    pub fn agenda_push(&mut self, item: Agendum) {
        self.agenda.push(item);
    }
    pub fn agenda_peek(&self) -> AgendaStatus {
        self.agenda.last().copied()
    }
}

pub enum ActorContext<'a> {
    OffRoad {
        pos_param: road::PosParam,
        segment_ctx: road::SegmentContext<'a>,
        segment_side: road::Direction,
        actor: &'a Actor,
    },
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
}

fn to_off_road_location(
    lane_ctx: road::SegmentLaneContext,
    pos_param: road::PosParam,
) -> LocationOffRoad {
    LocationOffRoad {
        segment_id: lane_ctx.segment_ctx.id,
        segment_side: lane_ctx.direction,
        pos_param: match lane_ctx.direction {
            road::Direction::Forward => pos_param,
            road::Direction::Backward => 1.0 - pos_param,
        },
    }
}
fn to_on_road_location(
    segment_ctx: &road::SegmentContext,
    segment_side: road::Direction,
    pos_param: road::PosParam,
) -> Result<LocationOnRoad, NoSuchLocationError> {
    use road::Direction::{Backward, Forward};
    let segment = segment_ctx.segment;
    let (lane_direction, lane_rank) =
        match (segment_side, segment.backward_lanes.len(), segment.forward_lanes.len()) {
            (_, 0, 0) => Err(NoSuchLocationError),
            (Backward, 0, _) => Ok((Forward, segment.forward_lanes.first_idx())),
            (Forward, _, 0) => Ok((Backward, segment.backward_lanes.last_idx())),
            (Forward, _, _) => Ok((Forward, segment.forward_lanes.last_idx())),
            (Backward, _, _) => Ok((Backward, segment.backward_lanes.last_idx())),
        }?;
    Ok(LocationOnRoad {
        segment_id: segment_ctx.id,
        lane_direction,
        lane_rank,
        pos_param,
    })
}

impl ActorContext<'_> {
    pub fn advance(&self, network: &mut road::Network) {
        match self {
            ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => {
                match actor.agenda_peek() {
                    None => {}
                    Some(agendum) => {
                        match agendum {
                            Agendum::SleepFor(time) => todo!(),
                            Agendum::TravelTo(destination) => {
                                let mut actor_next = (*actor).clone();

                                // setup route. todo: actual pathfinding, build route
                                actor_next.route_push(RouteStep::ArriveAt(destination.pos_param));

                                // move onto road
                                let start_location =
                                    to_on_road_location(segment_ctx, *segment_side, *pos_param).unwrap();
                                let lane = network
                                    .segments
                                    .get_mut(&start_location.segment_id)
                                    .unwrap()
                                    .get_lanes_mut(start_location.lane_direction)
                                    .get_mut(&start_location.lane_rank)
                                    .unwrap();
                                lane.actors.insert(start_location.pos_param, actor_next);
                            }
                        }
                    }
                }
            }
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
                        // todo: move off road
                    }
                    Some(step) => match step {
                        RouteStep::ArriveAt(pos_param_target) => {
                            let mut actor_next = (*actor).clone();
                            if pos_param_next_naive >= pos_param_target {
                                actor_next.route_pop().unwrap();
                                lane_current.actors.insert(pos_param_target, actor_next);
                            } else {
                                lane_current.actors.insert(pos_param_next_naive, actor_next);
                            }
                        }
                        RouteStep::LaneChange(lane_rank) => todo!(),
                        RouteStep::TurnOnto(lane_id) => {
                            if pos_param_next_naive > 1.0 {
                                todo!()
                            } else {
                                lane_current.actors.insert(pos_param_next_naive, (*actor).clone());
                            }
                        }
                    },
                }
            }
            ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => todo!(),
        }
    }
}
