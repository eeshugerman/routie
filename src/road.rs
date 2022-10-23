use std::collections::HashSet;

use nalgebra::Point2;

use crate::actor::Actor;

pub struct RoadJunction<'a> {
    pub pos: Point2<f64>,
    // or HashSet? should order matter? Would probably want an ID...
    // pub segments: HashSet<&'a RoadSegment<'a>>,
    pub segments: Vec<&'a RoadSegment<'a>>,
    // lane_inputs: HashTable<>,
    // lane_outputs: HashTable<>
}

pub struct RoadSegment<'a> {
    /// off-road only, otherwise they belong to lanes
    // actors: BTreeMap<PosParam, Actor>,
    pub begin_junction: &'a RoadJunction<'a>,
    pub end_junction: &'a RoadJunction<'a>,
    // forward_lanes: Vec<RoadLane>,
    // backward_lanes: Vec<RoadLane>
}
