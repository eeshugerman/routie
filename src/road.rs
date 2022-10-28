use nalgebra::Point2;
use std::{
    collections::{HashMap, HashSet},
    sync::atomic,
};

#[derive(Debug)]
pub enum RoutieError {
    AlreadyLinkedSegment,
    InvalidId, // TODO: be more specific
    UnlinkedSegment
    // InternalError  // for things that should never happen
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct JunctionId(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct SegmentId(usize);

pub struct Network {
    id_source: atomic::AtomicUsize,
    junctions: HashMap<JunctionId, Junction>,
    segments: HashMap<SegmentId, Segment>,
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

    fn get_id(&self) -> usize {
        // TODO: can probably relax ordering -- https://doc.rust-lang.org/nightly/nomicon/atomics.html
        self.id_source.fetch_add(1, atomic::Ordering::SeqCst)
    }

    pub fn register_junction(&mut self, pos: Point2<f64>) -> JunctionId {
        let id = JunctionId(self.get_id());
        self.junctions.insert(id, Junction { id, pos });
        id
    }
    pub fn register_segment(
        &mut self,
        forward_lanes: Vec<SegmentLane>,
        backward_lanes: Vec<SegmentLane>,
    ) -> SegmentId {
        let id = SegmentId(self.get_id());
        self.segments.insert(
            id,
            Segment {
                id,
                forward_lanes,
                backward_lanes,
            },
        );
        id
    }
    pub fn link(
        &mut self,
        segment: SegmentId,
        begin_junction: JunctionId,
        end_junction: JunctionId,
    ) -> Result<(), RoutieError> {
        if !self
            .junction_segments
            .entry(begin_junction)
            .or_insert(HashSet::new())
            .insert(segment)
        {
            log::warn!("Segment loops! Is this what you want?");
        };
        if !self
            .junction_segments
            .entry(end_junction)
            .or_insert(HashSet::new())
            .insert(segment)
        {
            log::warn!("Segment loops! Is this what you want?");
        };

        match self
            .segment_junctions
            .insert(segment, (begin_junction, end_junction))
        {
            Some(_) => Err(RoutieError::AlreadyLinkedSegment), // TODO: relink instead?
            None => Ok(()),
        }
    }
    pub fn get_segment_junctions(
        &self,
        segment: SegmentId,
    ) -> Result<(&Junction, &Junction), RoutieError> {
        let ids_maybe = self.segment_junctions.get(&segment);
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

    pub fn junctions(&self) -> &HashMap<JunctionId, Junction> {
        // TODO: maybe just return the `Junction`s in a Vec
        &self.junctions
    }

    pub fn segments(&self) -> &HashMap<SegmentId, Segment> {
        // TODO: maybe just return the `Segment`s in a Vec
        &self.segments
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

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
