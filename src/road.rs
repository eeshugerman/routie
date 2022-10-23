use std::collections::HashSet;

use nalgebra::Point2;

use crate::actor::Actor;

pub struct RoadJunction {
    pos: Point2<f64>,
    segments: HashSet<RoadSegment>,
    // lane_inputs: HashTable<>,
    // lane_outputs: HashTable<>
}

pub struct RoadSegment {
    actors: HashSet<Actor>,  // off-road only, otherwise they belong to lanes
    begin_junction: RoadJunction,
    end_junction: RoadJunction,
    // forward_lanes: Vec<RoadLane>,
    // backward_lanes: Vec<RoadLane>
}
