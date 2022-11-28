use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{error::RoutieError, spatial::Pos, util::seq_indexed_store::{self, SeqIndexedStore}};

#[derive(Debug)]
pub struct Actor {}

#[derive(Debug)]
pub struct PosParam(f64);

#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
}

define_index_type!(JunctionId);
define_index_type!(SegmentId);

#[derive(Debug)]
pub struct Junction {
    pub pos: Pos,
}

#[derive(Debug)]
pub struct SegmentLane {
    actors: BTreeMap<PosParam, Actor>,
    pub direction: Direction,
}

pub struct Segment {
    /// off-road only, otherwise they belong to lanes
    pub actors: BTreeMap<PosParam, Actor>,
    pub forward_lanes: SeqIndexedStore<usize, SegmentLane>, // TODO: LaneRank newtype?
    pub backward_lanes: SeqIndexedStore<usize, SegmentLane>,
}

pub struct Network {
    junctions: SeqIndexedStore<JunctionId, Junction>,
    segments: SeqIndexedStore<SegmentId, Segment>,
    junction_segments: HashMap<JunctionId, HashSet<SegmentId>>,
    segment_junctions: HashMap<SegmentId, (JunctionId, JunctionId)>,
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
        self.junctions.push(Junction { pos })
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
    pub rank: usize, // TODO: newtype?
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
    pub fn new(segment: &'a SegmentContext<'a>, rank: usize, lane: &'a SegmentLane) -> Self {
        Self {
            segment,
            rank,
            itself: lane,
        }
    }
}
