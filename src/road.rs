use std::collections::HashSet;

use nalgebra::Point2;

use crate::actor::Actor;

pub struct RoadJunction {
    pub pos: Point2<f64>,
    pub segments: Vec<RoadSegment>,
    // lane_inputs: HashTable<>,
    // lane_outputs: HashTable<>
}

pub struct RoadSegment {
    /// off-road only, otherwise they belong to lanes
    actors: Vec<Actor>,
    begin_junction: RoadJunction,
    end_junction: RoadJunction,
    // forward_lanes: Vec<RoadLane>,
    // backward_lanes: Vec<RoadLane>
}
