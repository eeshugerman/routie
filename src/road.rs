use std::collections::HashSet;

use nalgebra::Point2;

use crate::actor::Actor;

pub struct RoadJunction {
    pub pos: Point2<f64>,
    // pub segments: HashSet<&RoadSegment>, // or Vec? should order matter?
    // lane_inputs: HashTable<>,
    // lane_outputs: HashTable<>
}

pub struct RoadSegment<'a> {
    /// off-road only, otherwise they belong to lanes
    // actors: BTreeMap<PosParam, Actor>,
    pub begin_junction: &'a RoadJunction,
    pub end_junction: &'a RoadJunction,
    // forward_lanes: Vec<RoadLane>,
    // backward_lanes: Vec<RoadLane>
}
