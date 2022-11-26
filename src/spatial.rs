use std::f64::consts::FRAC_PI_2;

use nalgebra::{Point2, Rotation2, Vector2};

use crate::{constants::ROAD_LANE_WIDTH, error::RoutieError, road::Direction};

pub type Pos = Point2<f64>;
pub type Vector = Vector2<f64>;

pub mod located {
    use crate::road;
    pub struct Junction<'a>(pub &'a road::Junction);

    // TODO: use named structs?
    pub struct Segment<'a>(pub &'a road::Network, pub &'a road::Segment);

    pub struct SegmentLane<'a>(pub &'a Segment<'a>, pub &'a road::SegmentLane);

    // impl Segment
}

pub trait PointLike {
    fn get_pos(&self) -> Pos;
}

pub trait LineLike {
    fn get_pos(&self) -> (Pos, Pos);
    fn get_v_norm(&self) -> Vector {
        let (begin_pos, end_pos) = self.get_pos();
        end_pos - begin_pos
    }
    fn get_v_ortho(&self) -> Vector {
        let (begin_pos, end_pos) = self.get_pos();
        let vec = (end_pos - begin_pos).normalize();
        let rot = Rotation2::new(FRAC_PI_2);
        return rot * vec;
    }
}

impl<'a> PointLike for located::Junction<'a> {
    fn get_pos(&self) -> Pos {
        let located::Junction(junction) = self;
        junction.pos
    }
}

impl<'a> LineLike for located::Segment<'a> {
    fn get_pos(&self) -> (Pos, Pos) {
        let located::Segment(network, segment) = self;
        let (begin_junction, end_junction) = network
            .get_segment_junctions(segment)
            .expect(format!("Unlinked segment {:?}", segment.id).as_str());
        (begin_junction.pos, end_junction.pos)
    }
}
impl<'a> LineLike for located::SegmentLane<'a> {
    fn get_v_norm(&self) -> Vector {
        let located::SegmentLane(_, lane) = self;
        let (segment_begin_pos, segment_end_pos) = self.get_pos();
        match lane.direction {
            Direction::Backward => segment_begin_pos - segment_end_pos,
            Direction::Forward => segment_end_pos - segment_begin_pos,
        }
    }
    fn get_pos(&self) -> (Pos, Pos) {
        let located::SegmentLane(located_segment @ located::Segment(_, segment), lane) = self;
        let (segment_begin_pos, segment_end_pos) = located_segment.get_pos();
        let v_offset = {
            let lane_count_from_edge = match lane.direction {
                Direction::Backward => segment.backward_lanes.len() - lane.rank - 1,
                Direction::Forward => segment.backward_lanes.len() + lane.rank,
            };
            let v_ortho = located_segment.get_v_ortho();
            let v_segment_edge = (-0.5)
                * ROAD_LANE_WIDTH
                * (segment.backward_lanes.len() + segment.forward_lanes.len()) as f64
                * v_ortho;
            let v_lane_edge = v_segment_edge
                + (lane_count_from_edge as f64 * ROAD_LANE_WIDTH * v_ortho);
            v_lane_edge + (0.5 * ROAD_LANE_WIDTH * v_ortho)
        };
        match lane.direction {
            Direction::Backward => (segment_end_pos + v_offset, segment_begin_pos + v_offset),
            Direction::Forward => (segment_begin_pos + v_offset, segment_end_pos + v_offset)
        }
    }
}
