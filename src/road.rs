use nalgebra::Point2;
use std::{
    collections::{hash_map, HashMap, HashSet},
    sync::atomic,
};

use crate::error::RoutieError;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct JunctionId(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct SegmentId(usize);

pub struct Network {
    id_source: atomic::AtomicUsize,
    junctions: HashMap<JunctionId, Junction>, // these two should maybe just be vectors
    segments: HashMap<SegmentId, Segment>,    // then we wouldn't need `generate_id`
    junction_segments: HashMap<JunctionId, HashSet<SegmentId>>,
    segment_junctions: HashMap<SegmentId, (JunctionId, JunctionId)>,
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

    pub fn add_junction(&mut self, pos: Point2<f64>) -> JunctionId {
        let id = JunctionId(self.generate_id());
        self.junctions.insert(id, Junction { id, pos });
        id
    }
    pub fn add_segment(
        &mut self,
        begin_junction: JunctionId,
        end_junction: JunctionId,
    ) -> &Segment {
        let id = SegmentId(self.generate_id());
        self.segments.insert(
            id,
            Segment {
                id,
                forward_lanes: vec![],
                backward_lanes: vec![],
            },
        );
        self.segment_junctions
            .insert(id, (begin_junction, end_junction));
        for junction in [begin_junction, end_junction].iter() {
            if !self
                .junction_segments
                .entry(*junction)
                .or_insert(HashSet::new())
                .insert(id)
            {
                log::warn!("Segment loops! Is this what you want?");
            };
        }
        self.segments.get(&id).unwrap()
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
        let ids_maybe = self.segment_junctions.get(&segment.id);
        match ids_maybe {
            None => Err(RoutieError::InvalidId),
            Some((begin_id, end_id)) => {
                match (self.junctions.get(begin_id), self.junctions.get(end_id)) {
                    (Some(begin_junction), Some(end_junction)) => {
                        Ok((begin_junction, end_junction))
                    }
                    (_, _) => Err(RoutieError::InvalidId),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Junction {
    id: JunctionId,
    pub pos: Point2<f64>,
}

pub struct Segment {
    pub id: SegmentId,
    /// off-road only, otherwise they belong to lanes
    // actors: BTreeMap<PosParam, Actor>,
    forward_lanes: Vec<SegmentLane>,
    backward_lanes: Vec<SegmentLane>,
    // pub begin_junction: &Junction,
    // pub end_junction: &Junction,
}

pub struct SegmentLane {
    //
}
