use std::{
    collections::{BTreeMap, HashMap, HashSet},
    iter::{Enumerate, Map},
    marker::PhantomData,
    sync::atomic,
};

use crate::{error::RoutieError, spatial::Pos};

pub struct VecMap<U, T> {
    index_type: PhantomData<U>,
    data: Vec<T>,
}

impl<U: From<usize> + Into<usize>, T> VecMap<U, T> {
    fn new() -> Self {
        Self {
            index_type: PhantomData,
            data: Vec::new(),
        }
    }
    fn push(&mut self, val: T) -> U {
        let id = self.data.len();
        self.data.push(val);
        U::from(id)
    }
    fn get(&self, id: U) -> Option<&T> {
        self.data.get(id.into())
    }
    fn get_mut(&mut self, id: U) -> Option<&mut T> {
        self.data.get_mut(id.into())
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    // TODO: implement IntoIter instead
    pub fn enumerate(&self) -> Map<Enumerate<std::slice::Iter<'_, T>>, fn((usize, &T)) -> (U, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|val| (U::from(val.0), &val.1))
    }
}

#[derive(Debug)]
pub struct Actor {}

#[derive(Debug)]
pub struct PosParam(f64);

#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
}

// TODO: can we do something so that values don't need to be referenced? remove Clone?
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct JunctionId(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct SegmentId(usize);

impl From<usize> for JunctionId {
    fn from(id: usize) -> JunctionId {
        JunctionId(id)
    }
}
impl From<JunctionId> for usize {
    fn from(id: JunctionId) -> usize {
        id.0
    }
}
impl From<usize> for SegmentId {
    fn from(id: usize) -> SegmentId {
        SegmentId(id)
    }
}
impl From<SegmentId> for usize {
    fn from(id: SegmentId) -> usize {
        id.0
    }
}


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
    pub forward_lanes: VecMap<usize, SegmentLane>, // TODO: LaneRank newtype?
    pub backward_lanes: VecMap<usize, SegmentLane>,
}

pub struct Network {
    id_source: atomic::AtomicUsize,
    junctions: VecMap<JunctionId, Junction>,
    segments: VecMap<SegmentId, Segment>,
    junction_segments: HashMap<JunctionId, HashSet<SegmentId>>,
    segment_junctions: HashMap<SegmentId, (JunctionId, JunctionId)>,
}

impl Segment {
    pub fn new() -> Self {
        Self {
            forward_lanes: VecMap::new(),
            backward_lanes: VecMap::new(),
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
            id_source: atomic::AtomicUsize::new(0),
            junctions: VecMap::new(),
            segments: VecMap::new(),
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

    pub fn get_junctions(&self) -> &VecMap<JunctionId, Junction> {
        &self.junctions
    }

    pub fn get_segments(&self) -> &VecMap<SegmentId, Segment> {
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

pub mod context {
    use crate::road;
    pub struct Junction<'a> {
        pub network: &'a road::Network,
        pub id: road::JunctionId,
        pub itself: &'a road::Junction,
    }
    pub struct Segment<'a> {
        pub network: &'a road::Network,
        pub id: road::SegmentId,
        pub itself: &'a road::Segment,
    }
    pub struct SegmentLane<'a> {
        pub segment: &'a Segment<'a>,
        pub rank: usize, // TODO: newtype?
        pub itself: &'a road::SegmentLane,
    }

    impl<'a> Junction<'a> {
        pub fn new(
            network: &'a road::Network,
            id: road::JunctionId,
            junction: &'a road::Junction,
        ) -> Self {
            Self {
                network,
                id,
                itself: junction,
            }
        }
    }
    impl<'a> Segment<'a> {
        pub fn new(
            network: &'a road::Network,
            id: road::SegmentId,
            segment: &'a road::Segment,
        ) -> Self {
            Self {
                network,
                id,
                itself: segment,
            }
        }
    }
    impl<'a> SegmentLane<'a> {
        pub fn new(segment: &'a Segment<'a>, rank: usize, lane: &'a road::SegmentLane) -> Self {
            Self {
                segment,
                rank,
                itself: lane,
            }
        }
    }
}
