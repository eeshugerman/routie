extern crate nalgebra;
extern crate pathfinding;

use crate::{constants, road};

#[derive(Clone, Copy, Debug)]
pub enum Agendum {
    SleepFor(i32),
    TravelTo {
        segment_id: road::SegmentId,
        segment_side: road::Direction,
        pos_param: road::PosParam,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum RouteStep {
    ArriveAt(f64),
    LaneChange(road::SegmentLaneRank),
    TurnAt(road::JunctionLaneId),
}

#[derive(Clone, Debug)]
pub struct Actor {
    max_speed: f64,
    route: Vec<RouteStep>,
    agenda: Vec<Agendum>,
}

// TODO: clean this up
#[derive(Debug)]
pub struct NullRouteError;
#[derive(Debug)]
pub struct NullAgendaError;
#[derive(Debug)]
struct NoSuchLocationError;

type AgendaStatus = Option<Agendum>;

impl Actor {
    pub fn new(agenda: Vec<Agendum>) -> Self {
        Self { max_speed: constants::ACTOR_MAX_SPEED, agenda, route: Vec::new() }
    }

    pub fn route_push(&mut self, item: RouteStep) {
        self.route.push(item)
    }

    pub fn route_peek(&self) -> Option<RouteStep> {
        self.route.last().copied()
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
    pub fn agenda_pop(&mut self) -> Result<Agendum, NullAgendaError> {
        if self.agenda.len() > 0 {
            Ok(self.agenda.remove(self.agenda.len() - 1))
        } else {
            Err(NullAgendaError)
        }
    }
}

pub enum ActorContext<'a> {
    OffRoad {
        pos_param: road::PosParam,
        segment_ctx: &'a road::SegmentContext<'a>,
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

fn to_on_road_location(
    segment_ctx: &road::SegmentContext,
    segment_side: road::Direction,
    pos_param: road::PosParam,
) -> Result<(road::Direction, road::SegmentLaneRank, road::PosParam), NoSuchLocationError> {
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
    let pos_param = if segment_side == lane_direction { pos_param } else { 1.0 - pos_param };
    Ok((lane_direction, lane_rank, pos_param))
}

impl ActorContext<'_> {
    pub fn advance(&self, network_pp: &mut road::Network) {
        // naming conventions:
        // - road componenets and actors may be undecorated (current world) or _pp ("plus-plus") (next world)
        // - road components and scalars may be undecorated (current) or _next
        match self {
            ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => {
                match actor.agenda_peek() {
                    None => {
                        // stay put
                        let segment_pp = network_pp.segments.get_mut(&segment_ctx.id).unwrap();
                        match segment_side {
                            road::Direction::Forward => &mut segment_pp.forward_actors,
                            road::Direction::Backward => &mut segment_pp.backward_actors,
                        }
                        .insert(*pos_param, (*actor).clone())
                    }
                    Some(agendum) => {
                        match agendum {
                            Agendum::SleepFor(time) => todo!(),
                            Agendum::TravelTo {
                                segment_id: segment_id_dest,
                                segment_side: segment_side_dest,
                                pos_param: pos_param_dest,
                            } => {
                                let mut actor_pp = (*actor).clone();
                                actor_pp.agenda_pop().unwrap();

                                // setup route. TODO: actual pathfinding, build route
                                // if pos_param_dest < *pos_param {
                                //     actor_pp.route_push(RouteStep::ArriveAt(pos_param_dest));
                                //     actor_pp.route_push(RouteStep::TurnAt(0.into()));
                                // } else {
                                //     actor_pp.route_push(RouteStep::ArriveAt(pos_param_dest));
                                // }

                                let (lane_direction_next, lane_rank_next, pos_param_next) =
                                    to_on_road_location(segment_ctx, *segment_side, *pos_param)
                                        .unwrap();

                                let start: road::QualifiedSegmentLaneRank =
                                    (segment_ctx.id, lane_direction_next, lane_rank_next);
                                let route_raw = pathfinding::prelude::astar(
                                    start,
                                    |segment_lane @ (segment_id, direction, rank): road::QualifiedSegmentLaneRank| -> Vec<(road::QualifiedSegmentLaneRank, u32)>{
                                        let (begin_junction_id, end_junction_id) = network_pp.get_segment_junctions(segment_id).unwrap();
                                        let junction = network_pp.junctions.get(
                                            &match direction {
                                                road::Direction::Forward => end_junction_id,
                                                road::Direction::Backward => begin_junction_id,
                                            }
                                        ).unwrap();
                                        const cost: u32 = 1; // TODO
                                        junction.get_outputs_for_input(segment_lane).into_iter().map(|step| (step, cost)).collect()
                                    }
                                );

                                let lane_next_pp = network_pp
                                    .segments
                                    .get_mut(&segment_ctx.id)
                                    .unwrap()
                                    .get_lanes_mut(lane_direction_next)
                                    .get_mut(&lane_rank_next)
                                    .unwrap();
                                lane_next_pp.actors.insert(pos_param_next, actor_pp);
                            }
                        }
                    }
                }
            }
            ActorContext::OnRoadSegment { pos_param, lane_ctx, actor } => {
                let mut actor_pp = (*actor).clone();
                let segment_pp = network_pp.segments.get_mut(&lane_ctx.segment_ctx.id).unwrap();
                let lane_pp =
                    segment_pp.get_lanes_mut(lane_ctx.direction).get_mut(&lane_ctx.rank).unwrap();
                // TODO: account for lane length
                let pos_param_next_naive = pos_param + actor.max_speed * constants::SIM_TIME_STEP;
                match actor.route_peek() {
                    None => {
                        // done, move off road
                        match lane_ctx.direction {
                            road::Direction::Forward => &mut segment_pp.forward_actors,
                            road::Direction::Backward => &mut segment_pp.backward_actors,
                        }
                        .insert(*pos_param, actor_pp)
                    }
                    Some(step) => match step {
                        RouteStep::ArriveAt(pos_param_target) => {
                            if pos_param_next_naive >= pos_param_target {
                                actor_pp.route_pop().unwrap();
                                lane_pp.actors.insert(pos_param_target, actor_pp);
                            } else {
                                lane_pp.actors.insert(pos_param_next_naive, actor_pp);
                            }
                        }
                        RouteStep::LaneChange(lane_rank) => todo!(),
                        RouteStep::TurnAt(lane_id) => {
                            if pos_param_next_naive > 1.0 {
                                let (begin_junction_id, end_junction_id) = network_pp
                                    .get_segment_junctions(lane_ctx.segment_ctx.id)
                                    .unwrap();
                                let junction_pp = network_pp
                                    .junctions
                                    .get_mut(&match lane_ctx.direction {
                                        road::Direction::Backward => begin_junction_id,
                                        road::Direction::Forward => end_junction_id,
                                    })
                                    .unwrap();
                                let lane_pp = junction_pp.lanes.get_mut(&lane_id).unwrap();
                                lane_pp.actors.insert(pos_param_next_naive - 1.0, actor_pp)
                            } else {
                                lane_pp.actors.insert(pos_param_next_naive, actor_pp);
                            }
                        }
                    },
                }
            }
            ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => {
                let mut actor_pp = (*actor).clone();
                // TODO: account for lane length
                let pos_param_next_naive = pos_param + actor.max_speed * constants::SIM_TIME_STEP;
                if pos_param_next_naive > 1.0 {
                    actor_pp.route_pop().unwrap();
                    let (_, (segment_id, direction, segment_lane_rank)) = lane_ctx
                        .junction_ctx
                        .junction
                        .get_segment_lanes_for_junction_lane(lane_ctx.id);
                    let segment_pp = network_pp.segments.get_mut(&segment_id).unwrap();
                    let lane_pp = match direction {
                        road::Direction::Backward => &mut segment_pp.backward_lanes,
                        road::Direction::Forward => &mut segment_pp.forward_lanes,
                    }
                    .get_mut(&segment_lane_rank)
                    .unwrap();
                    lane_pp.actors.insert(pos_param_next_naive - 1.0, actor_pp)
                } else {
                    let lane_pp = network_pp
                        .junctions
                        .get_mut(&lane_ctx.junction_ctx.id)
                        .unwrap()
                        .lanes
                        .get_mut(&lane_ctx.id)
                        .unwrap();
                    lane_pp.actors.insert(pos_param_next_naive, actor_pp);
                }
            }
        }
    }
}
