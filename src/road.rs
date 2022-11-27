use std::{
    collections::{hash_map, BTreeMap, HashMap, HashSet},
    marker::PhantomData,
    sync::atomic,
};

use crate::{error::RoutieError, spatial::Pos};

struct VecMap<U, T> {
    index_type: PhantomData<U>,
    data: Vec<T>,
}

impl<U: From<usize> + Into<usize>, T> VecMap<U, T> {
    fn push(&mut self, val: T) -> U {
        let id = self.data.len();
        self.data.push(val);
        U::from(id)
    }
    fn get(&self, id: U) -> &T {
        &self.data[id.into()]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct JunctionId(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct SegmentId(usize);

#[derive(Debug)]
pub struct Junction {
    id: JunctionId,
    pub pos: Pos,
}

#[derive(Debug)]
pub struct SegmentLane {
    actors: BTreeMap<PosParam, Actor>,
    pub direction: Direction,
    pub rank: usize,
}

#[derive(Debug)]
pub struct Segment {
    pub id: SegmentId,
    /// off-road only, otherwise they belong to lanes
    pub actors: BTreeMap<PosParam, Actor>,
    pub forward_lanes: Vec<SegmentLane>,
    pub backward_lanes: Vec<SegmentLane>,
}

pub struct Network {
    id_source: atomic::AtomicUsize,
    junctions: HashMap<JunctionId, Junction>, // these two should maybe just be vectors
    segments: HashMap<SegmentId, Segment>,    // then we wouldn't need `generate_id`
    junction_segments: HashMap<JunctionId, HashSet<SegmentId>>,
    segment_junctions: HashMap<SegmentId, (JunctionId, JunctionId)>,
}

impl Segment {
    pub fn new(id: SegmentId) -> Self {
        Self {
            id,
            forward_lanes: vec![],
            backward_lanes: vec![],
            actors: BTreeMap::new(),
        }
    }
    pub fn add_lane(&mut self, direction: Direction) -> &mut SegmentLane {
        let lanes = match direction {
            Direction::Forward => &mut self.forward_lanes,
            Direction::Backward => &mut self.backward_lanes,
        };
        lanes.push(SegmentLane::new(direction, lanes.len()));
        lanes.last_mut().unwrap()
    }
}

impl SegmentLane {
    pub fn new(direction: Direction, rank: usize) -> Self {
        Self {
            direction,
            rank,
            actors: BTreeMap::new(),
        }
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            id_source: atomic::AtomicUsize::new(0),
            junctions: HashMap::new(),
            segments: HashMap::new(),
            junction_segments: HashMap::new(),
            segment_junctions: HashMap::new(),
        }
    }

    fn generate_id(&self) -> usize {
        // TODO: can probably relax ordering -- https://doc.rust-lang.org/nightly/nomicon/atomics.html
        self.id_source.fetch_add(1, atomic::Ordering::SeqCst)
    }

    pub fn add_junction(&mut self, pos: Pos) -> JunctionId {
        let id = JunctionId(self.generate_id());
        self.junctions.insert(id, Junction { id, pos });
        id
    }
    pub fn add_segment(&mut self, begin_id: JunctionId, end_id: JunctionId) -> &mut Segment {
        let id = SegmentId(self.generate_id());
        self.segments.insert(id, Segment::new(id));
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
        self.segments.get_mut(&id).unwrap()
    }

    pub fn get_junctions(&self) -> hash_map::Values<JunctionId, Junction> {
        self.junctions.values()
    }

    pub fn get_segments(&self) -> hash_map::Values<SegmentId, Segment> {
        self.segments.values()
    }

    pub fn get_segment_junctions(
        &self,
        segment: &Segment,
    ) -> Result<(&Junction, &Junction), RoutieError> {
        match self.segment_junctions.get(&segment.id) {
            None => Err(RoutieError::InvalidId),
            Some((begin_id, end_id)) => {
                match (self.junctions.get(begin_id), self.junctions.get(end_id)) {
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
        pub junction: &'a road::Junction,
    }
    pub struct Segment<'a> {
        pub network: &'a road::Network,
        pub segment: &'a road::Segment,
    }
    pub struct SegmentLane<'a> {
        pub segment: &'a Segment<'a>,
        pub lane: &'a road::SegmentLane,
    }

    impl<'a> Junction<'a> {
        pub fn new(network: &'a road::Network, junction: &'a road::Junction) -> Self {
            Self { network, junction }
        }
    }
    impl<'a> Segment<'a> {
        pub fn new(network: &'a road::Network, segment: &'a road::Segment) -> Self {
            Self { network, segment }
        }
    }
    impl<'a> SegmentLane<'a> {
        pub fn new(segment: &'a Segment<'a>, lane: &'a road::SegmentLane) -> Self {
            Self { segment, lane }
        }
    }
}
