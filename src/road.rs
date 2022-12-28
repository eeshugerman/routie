use std::collections::{HashMap, HashSet};

use crate::{
    actor::Actor,
    error::RoutieError,
    spatial::Pos,
    util::{ordered_skip_map::OrderedSkipMap, seq_indexed_store::SeqIndexedStore, CloneEmpty},
};

pub type PosParam = f64;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

use Direction::{Backward, Forward};

define_index_type!(JunctionId);
define_index_type!(SegmentId);
define_index_type!(SegmentLaneRank);
define_index_type!(JunctionLaneId);

pub type QualifiedSegmentLaneRank = (SegmentId, Direction, SegmentLaneRank);

#[derive(Debug)]
pub struct Network {
    pub junctions: SeqIndexedStore<JunctionId, Junction>,
    pub segments: SeqIndexedStore<SegmentId, Segment>,
    junction_segments: HashMap<JunctionId, HashSet<SegmentId>>,
    segment_junctions: HashMap<SegmentId, (JunctionId, JunctionId)>,
}
#[derive(Debug)]
pub struct Junction {
    pub pos: Pos,
    lanes: SeqIndexedStore<JunctionLaneId, JunctionLane>,
    lane_inputs: HashMap<QualifiedSegmentLaneRank, HashSet<JunctionLaneId>>,
    lane_inputs_inverse: HashMap<JunctionLaneId, QualifiedSegmentLaneRank>,
    lane_outputs: HashMap<JunctionLaneId, QualifiedSegmentLaneRank>,
}
#[derive(Debug)]
pub struct JunctionLane {
    actors: OrderedSkipMap<PosParam, Actor>,
}
#[derive(Debug)]
pub struct Segment {
    /// off-road only, otherwise they belong to lanes
    actors: OrderedSkipMap<PosParam, Actor>,
    pub forward_lanes: SeqIndexedStore<SegmentLaneRank, SegmentLane>,
    pub backward_lanes: SeqIndexedStore<SegmentLaneRank, SegmentLane>,
}
#[derive(Debug)]
pub struct SegmentLane {
    pub direction: Direction, // TODO: remove this; it belongs to the context
    pub actors: OrderedSkipMap<PosParam, Actor>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            junctions: SeqIndexedStore::new(),
            segments: SeqIndexedStore::new(),
            junction_segments: HashMap::new(),
            segment_junctions: HashMap::new(),
        }
    }

    pub fn add_junction(&mut self, pos: Pos) -> JunctionId {
        self.junctions.push(Junction::new(pos))
    }

    pub fn add_segment(&mut self, begin_id: JunctionId, end_id: JunctionId) -> &mut Segment {
        let id = self.segments.push(Segment::new());
        self.segment_junctions.insert(id, (begin_id, end_id));
        for junction in [begin_id, end_id].iter() {
            if !self.junction_segments.entry(*junction).or_insert(HashSet::new()).insert(id) {
                log::warn!("Segment loops! Is this what you want?");
            };
        }
        self.segments.get_mut(&id).unwrap()
    }

    pub fn get_segment_junctions(
        &self,
        segment: SegmentId,
    ) -> Result<(JunctionId, JunctionId), RoutieError> {
        match self.segment_junctions.get(&segment) {
            None => Err(RoutieError::InvalidId),
            Some((begin_id, end_id)) => Ok((*begin_id, *end_id)),
        }
    }

    pub fn connect_junctions(&mut self) {
        for (junction_id, junction) in self.junctions.enumerate_mut() {
            let empty_set = HashSet::<SegmentId>::new();
            let segment_ids = match self.junction_segments.get(&junction_id) {
                Some(ids) => ids,
                None => {
                    log::warn!("Junction has no linked segments");
                    &empty_set
                }
            };
            for incoming_segment_id in segment_ids {
                let incoming_segment = self.segments.get(incoming_segment_id).unwrap();
                let incoming_direction = {
                    let (begin_junction_id, end_junction_id) =
                        *self.segment_junctions.get(&incoming_segment_id).unwrap();
                    assert!(junction_id == begin_junction_id || junction_id == end_junction_id);
                    // damn you rustfmt
                    if junction_id == begin_junction_id {
                        Backward
                    } else {
                        Forward
                    }
                };

                let incoming_lanes = incoming_segment.get_lanes(incoming_direction);
                for outgoing_segment_id in segment_ids {
                    if incoming_segment_id == outgoing_segment_id {
                        continue;
                    }
                    let outgoing_segment = self.segments.get(outgoing_segment_id).unwrap();
                    let outgoing_direction = {
                        let (begin_junction_id, end_junction_id) =
                            *self.segment_junctions.get(&outgoing_segment_id).unwrap();
                        assert!(junction_id == begin_junction_id || junction_id == end_junction_id);
                        if junction_id == begin_junction_id {
                            Forward
                        } else {
                            Backward
                        }
                    };
                    let outgoing_lanes = outgoing_segment.get_lanes(outgoing_direction);

                    // connect by rank //
                    for ((incoming_lane_rank, _), (outgoing_lane_rank, _)) in
                        std::iter::zip(incoming_lanes.enumerate(), outgoing_lanes.enumerate())
                    {
                        junction.add_lane(
                            (*incoming_segment_id, incoming_direction, incoming_lane_rank),
                            (*outgoing_segment_id, outgoing_direction, outgoing_lane_rank),
                        );
                    }
                }
            }
        }
    }
}
impl Junction {
    pub fn new(pos: Pos) -> Self {
        Self {
            pos,
            lanes: SeqIndexedStore::new(),
            lane_inputs: HashMap::new(),
            lane_inputs_inverse: HashMap::new(),
            lane_outputs: HashMap::new(),
        }
    }

    fn add_lane(
        &mut self,
        begin: QualifiedSegmentLaneRank,
        end: QualifiedSegmentLaneRank,
    ) -> &JunctionLane {
        let id = self.lanes.push(JunctionLane::new());
        self.lane_inputs.entry(begin).or_insert(HashSet::new()).insert(id);
        self.lane_inputs_inverse.insert(id, begin);
        self.lane_outputs.insert(id, end);
        self.lanes.get(&id).unwrap()
    }

    pub fn enumerate_lanes(&self) -> impl Iterator<Item = (JunctionLaneId, &JunctionLane)> {
        self.lanes.enumerate()
    }
}
impl JunctionLane {
    pub fn new() -> Self {
        Self { actors: OrderedSkipMap::new(Actor::new) }
    }
}
impl Segment {
    pub fn new() -> Self {
        Self {
            forward_lanes: SeqIndexedStore::new(),
            backward_lanes: SeqIndexedStore::new(),
            actors: OrderedSkipMap::new(Actor::new),
        }
    }
    pub fn add_lane(&mut self, direction: Direction) -> &mut SegmentLane {
        let lanes = match direction {
            Forward => &mut self.forward_lanes,
            Backward => &mut self.backward_lanes,
        };
        let id = lanes.push(SegmentLane::new(direction));
        lanes.get_mut(&id).unwrap()
    }
    pub fn get_lanes(
        &self,
        direction: Direction,
    ) -> &SeqIndexedStore<SegmentLaneRank, SegmentLane> {
        match direction {
            Forward => &self.forward_lanes,
            Backward => &self.backward_lanes,
        }
    }
    pub fn get_lanes_mut(
        &mut self,
        direction: Direction,
    ) -> &mut SeqIndexedStore<SegmentLaneRank, SegmentLane> {
        match direction {
            Forward => &mut self.forward_lanes,
            Backward => &mut self.backward_lanes,
        }
    }
}
impl SegmentLane {
    pub fn new(direction: Direction) -> Self {
        Self { direction, actors: OrderedSkipMap::new(Actor::new) }
    }
    pub fn add_actor(&mut self, pos_param: PosParam) {
        let actor = Actor::new();
        self.actors.insert(pos_param, actor);
    }
}

impl CloneEmpty for SegmentLane {
    fn clone_empty(&self) -> Self {
        Self { direction: self.direction, actors: OrderedSkipMap::new(Actor::new) }
    }
}
impl CloneEmpty for JunctionLane {
    fn clone_empty(&self) -> Self {
        Self { actors: OrderedSkipMap::new(Actor::new) }
    }
}
impl CloneEmpty for Segment {
    fn clone_empty(&self) -> Self {
        Self {
            forward_lanes: self.forward_lanes.clone_empty(),
            backward_lanes: self.backward_lanes.clone_empty(),
            actors: OrderedSkipMap::new(Actor::new),
        }
    }
}
impl CloneEmpty for Junction {
    fn clone_empty(&self) -> Self {
        Self {
            pos: self.pos,
            lanes: self.lanes.clone_empty(),
            lane_inputs: self.lane_inputs.clone(),
            lane_inputs_inverse: self.lane_inputs_inverse.clone(),
            lane_outputs: self.lane_outputs.clone(),
        }
    }
}
impl CloneEmpty for Network {
    fn clone_empty(&self) -> Self {
        Self {
            junctions: self.junctions.clone_empty(),
            segments: self.segments.clone_empty(),
            junction_segments: self.junction_segments.clone(),
            segment_junctions: self.segment_junctions.clone(),
        }
    }
}

pub struct JunctionContext<'a> {
    pub network: &'a Network,
    pub id: JunctionId,
    pub junction: &'a Junction,
}
pub struct JunctionLaneContext<'a> {
    pub junction: &'a JunctionContext<'a>,
    pub id: JunctionLaneId,
    pub lane: &'a JunctionLane,
}
pub struct SegmentContext<'a> {
    pub network: &'a Network,
    pub id: SegmentId,
    pub segment: &'a Segment,
}
pub struct SegmentLaneContext<'a> {
    pub segment_ctx: &'a SegmentContext<'a>,
    pub direction: Direction,
    pub rank: SegmentLaneRank,
    pub lane: &'a SegmentLane,
}

impl<'a> JunctionContext<'a> {
    pub fn new(network: &'a Network, id: JunctionId, junction: &'a Junction) -> Self {
        Self { network, id, junction }
    }
    pub fn get_segment_lanes_for_junction_lane(
        &self,
        lane_id: JunctionLaneId,
    ) -> (QualifiedSegmentLaneRank, QualifiedSegmentLaneRank) {
        let input_segment_lane = self.junction.lane_inputs_inverse.get(&lane_id).unwrap();
        let output_segment_lane = self.junction.lane_outputs.get(&lane_id).unwrap();
        (*input_segment_lane, *output_segment_lane)
    }
}
impl<'a> JunctionLaneContext<'a> {
    pub fn new(
        junction: &'a JunctionContext<'a>,
        id: JunctionLaneId,
        lane: &'a JunctionLane,
    ) -> Self {
        assert!(match junction.junction.lanes.get(&id) {
            None => false,
            Some(context_lane) => lane as *const _ == context_lane as *const _,
        });
        Self { junction, id, lane }
    }
}
impl<'a> SegmentContext<'a> {
    pub fn new(network: &'a Network, id: SegmentId, segment: &'a Segment) -> Self {
        Self { network, id, segment }
    }
    pub fn get_junctions(&self) -> (JunctionContext, JunctionContext) {
        let (begin_id, end_id) = self.network.get_segment_junctions(self.id).unwrap();
        let id_to_junc_ctx =
            |id| JunctionContext::new(self.network, id, self.network.junctions.get(&id).unwrap());
        (id_to_junc_ctx(begin_id), id_to_junc_ctx(end_id))
    }
}
impl<'a> SegmentLaneContext<'a> {
    pub fn new(
        segment_ctx: &'a SegmentContext<'a>,
        direction: Direction,
        rank: SegmentLaneRank,
        lane: &'a SegmentLane,
    ) -> Self {
        assert!(match segment_ctx.segment.get_lanes(direction).get(&rank) {
            None => false,
            Some(context_lane) => lane as *const _ == context_lane as *const _,
        });
        Self { segment_ctx, direction, rank, lane }
    }
}
