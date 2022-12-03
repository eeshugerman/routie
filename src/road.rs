use std::collections::{BTreeMap, HashMap, HashSet};

use cairo::glib::FormatSizeFlags;

use crate::{error::RoutieError, spatial::Pos, util::seq_indexed_store::SeqIndexedStore};

#[derive(Debug)]
pub struct Actor {}

#[derive(Debug)]
pub struct PosParam(f64);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

define_index_type!(JunctionId);
define_index_type!(SegmentId);
define_index_type!(SegmentLaneRank);
define_index_type!(JunctionLaneId);

pub struct SegmentLane {
    pub direction: Direction,
    actors: BTreeMap<PosParam, Actor>,
}

pub struct JunctionLane {
    actors: BTreeMap<PosParam, Actor>,
}

pub struct Segment {
    /// off-road only, otherwise they belong to lanes
    pub actors: BTreeMap<PosParam, Actor>,
    pub forward_lanes: SeqIndexedStore<SegmentLaneRank, SegmentLane>,
    pub backward_lanes: SeqIndexedStore<SegmentLaneRank, SegmentLane>,
}

// #[derive(PartialEq, Eq, Hash)]
pub type QualifiedSegmentLaneId = (SegmentId, Direction, SegmentLaneRank);

pub struct Junction {
    pub pos: Pos,
    lanes: SeqIndexedStore<JunctionLaneId, JunctionLane>,
    lane_inputs: HashMap<QualifiedSegmentLaneId, HashSet<JunctionLaneId>>,
    lane_outputs: HashMap<JunctionLaneId, QualifiedSegmentLaneId>,
}

pub struct Network {
    junctions: SeqIndexedStore<JunctionId, Junction>,
    segments: SeqIndexedStore<SegmentId, Segment>,
    junction_segments: HashMap<JunctionId, HashSet<SegmentId>>,
    segment_junctions: HashMap<SegmentId, (JunctionId, JunctionId)>,
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
            if !self
                .junction_segments
                .entry(*junction)
                .or_insert(HashSet::new())
                .insert(id)
            {
                log::warn!("Segment loops! Is this what you want?");
            };
        }
        self.segments.get_mut(id).unwrap()
    }

    pub fn get_junctions(&self) -> &SeqIndexedStore<JunctionId, Junction> {
        &self.junctions
    }

    pub fn get_segments(&self) -> &SeqIndexedStore<SegmentId, Segment> {
        &self.segments
    }

    pub fn get_segment_junctions(
        &self,
        segment: SegmentId,
    ) -> Result<(&Junction, &Junction), RoutieError> {
        match self.segment_junctions.get(&segment) {
            None => Err(RoutieError::InvalidId),
            Some((begin_id, end_id)) => {
                match (self.junctions.get(*begin_id), self.junctions.get(*end_id)) {
                    (Some(begin), Some(end)) => Ok((begin, end)),
                    (_, _) => Err(RoutieError::InvalidId),
                }
            }
        }
    }

    pub fn connect_junctions(&mut self) {
        for (junction_id, junction) in self.junctions.enumerate_mut() {
            let segment_ids = if let Some(ids) = self.junction_segments.get(&junction_id) {
                ids
            } else {
                log::warn!("Junction has no linked segments");
                return;
            };
            for incoming_segment_id in segment_ids {
                let incoming_segment = self.segments.get(*incoming_segment_id).unwrap();
                let (begin_junction_id, end_junction_id) =
                    *self.segment_junctions.get(&incoming_segment_id).unwrap();
                assert!(junction_id == begin_junction_id || junction_id == end_junction_id);
                let (incoming_direction, outgoing_direction) = if junction_id == begin_junction_id {
                    (Direction::Backward, Direction::Forward)
                } else {
                    (Direction::Forward, Direction::Backward)
                };
                let incoming_lanes = match incoming_direction {
                    Direction::Backward => &incoming_segment.backward_lanes,
                    Direction::Forward => &incoming_segment.forward_lanes,
                };
                for outgoing_segment_id in segment_ids {
                    if incoming_segment_id == outgoing_segment_id {
                        continue;
                    }
                    let outgoing_segment = self.segments.get(*outgoing_segment_id).unwrap();
                    let outgoing_lanes = &match outgoing_direction {
                        Direction::Backward => &outgoing_segment.backward_lanes,
                        Direction::Forward => &outgoing_segment.forward_lanes,
                    };
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
            lane_outputs: HashMap::new(),
        }
    }

    fn add_lane(
        &mut self,
        begin: QualifiedSegmentLaneId,
        end: QualifiedSegmentLaneId,
    ) -> &JunctionLane {
        let id = self.lanes.push(JunctionLane::new());
        self.lane_inputs
            .entry(begin)
            .or_insert(HashSet::new())
            .insert(id);
        self.lane_outputs.insert(id, end);
        self.lanes.get(id).unwrap()
    }
}

impl Segment {
    pub fn new() -> Self {
        Self {
            forward_lanes: SeqIndexedStore::new(),
            backward_lanes: SeqIndexedStore::new(),
            actors: BTreeMap::new(),
        }
    }
    pub fn add_lane(&mut self, direction: Direction) -> &mut SegmentLane {
        let lanes = match direction {
            Direction::Forward => &mut self.forward_lanes,
            Direction::Backward => &mut self.backward_lanes,
        };
        let id = lanes.push(SegmentLane::new(direction));
        lanes.get_mut(id).unwrap()
    }
}

impl SegmentLane {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            actors: BTreeMap::new(),
        }
    }
}

impl JunctionLane {
    pub fn new() -> Self {
        Self {
            actors: BTreeMap::new(),
        }
    }
}

pub struct JunctionContext<'a> {
    pub network: &'a Network,
    pub id: JunctionId,
    pub itself: &'a Junction,
}
pub struct SegmentContext<'a> {
    pub network: &'a Network,
    pub id: SegmentId,
    pub itself: &'a Segment,
}
pub struct SegmentLaneContext<'a> {
    pub segment: &'a SegmentContext<'a>,
    pub direction: Direction,
    pub rank: SegmentLaneRank,
    pub itself: &'a SegmentLane,
}

impl<'a> JunctionContext<'a> {
    pub fn new(network: &'a Network, id: JunctionId, junction: &'a Junction) -> Self {
        Self {
            network,
            id,
            itself: junction,
        }
    }
}
impl<'a> SegmentContext<'a> {
    pub fn new(network: &'a Network, id: SegmentId, segment: &'a Segment) -> Self {
        Self {
            network,
            id,
            itself: segment,
        }
    }
}
impl<'a> SegmentLaneContext<'a> {
    pub fn new(
        segment: &'a SegmentContext<'a>,
        direction: Direction,
        rank: SegmentLaneRank,
        lane: &'a SegmentLane,
    ) -> Self {
        let context_lanes = match direction {
            Direction::Forward => &segment.itself.forward_lanes,
            Direction::Backward => &segment.itself.backward_lanes,
        };
        assert!(match context_lanes.get(rank) {
            None => false,
            Some(context_lane) => lane as *const _ == context_lane as *const _,
        });
        Self {
            segment,
            direction,
            rank,
            itself: lane,
        }
    }
    pub fn get_qualified_id(self) -> QualifiedSegmentLaneId {
        (self.segment.id, self.direction, self.rank)
    }
}
