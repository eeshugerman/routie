use std::f64::consts::{FRAC_PI_2, PI};

use nalgebra::{Point2, Rotation2, Vector2};

use crate::{
    constants::{ROAD_LANE_WIDTH, ROAD_SEGMENT_WIGGLE_ROOM_PCT},
    road::Direction::{Backward, Forward},
};

pub type Pos = Point2<f64>;
pub type Vector = Vector2<f64>;

pub mod located {
    use crate::road;
    pub struct Junction<'a> {
        pub network: &'a road::Network,
        pub itself: &'a road::Junction,
    }
    pub struct Segment<'a> {
        pub network: &'a road::Network,
        pub itself: &'a road::Segment,
    }
    pub struct SegmentLane<'a> {
        pub segment: &'a Segment<'a>,
        pub itself: &'a road::SegmentLane,
    }

    impl<'a> Junction<'a> {
        pub fn new(network: &'a road::Network, itself: &'a road::Junction) -> Self {
            Self { network, itself }
        }
    }
    impl<'a> Segment<'a> {
        pub fn new(network: &'a road::Network, itself: &'a road::Segment) -> Self {
            Self { network, itself }
        }
    }
    impl<'a> SegmentLane<'a> {
        pub fn new(segment: &'a Segment<'a>, itself: &'a road::SegmentLane) -> Self {
            Self { segment, itself }
        }
    }
}

pub trait PointLike {
    fn get_pos(&self) -> Pos;
}

impl<'a> PointLike for located::Junction<'a> {
    fn get_pos(&self) -> Pos {
        self.itself.pos
    }
}

pub trait LineLike {
    fn get_width(&self) -> f64;
    fn get_pos(&self) -> (Pos, Pos);
    fn get_midpoint(&self) -> Pos {
        self.get_pos().0 + (0.5 * self.get_v())
    }
    fn get_v(&self) -> Vector {
        let (begin_pos, end_pos) = self.get_pos();
        end_pos - begin_pos
    }
    fn get_v_norm(&self) -> Vector {
        self.get_v().normalize()
    }
    fn get_v_ortho(&self) -> Vector {
        let rot = Rotation2::new(FRAC_PI_2);
        return rot * self.get_v_norm();
    }
}

impl<'a> LineLike for located::Segment<'a> {
    fn get_width(&self) -> f64 {
        let total_lane_count = self.itself.forward_lanes.len() + self.itself.backward_lanes.len();
        (1.0 + (ROAD_SEGMENT_WIGGLE_ROOM_PCT as f64 / 100.0))
            * ROAD_LANE_WIDTH
            * std::cmp::max(total_lane_count, 1) as f64
    }

    fn get_pos(&self) -> (Pos, Pos) {
        let (begin_junction, end_junction) = self
            .network
            .get_segment_junctions(self.itself)
            .expect(format!("Unlinked segment {:?}", self.itself.id).as_str());
        (begin_junction.pos, end_junction.pos)
    }
}
impl<'a> LineLike for located::SegmentLane<'a> {
    fn get_width(&self) -> f64 {
        ROAD_LANE_WIDTH
    }
    fn get_v(&self) -> Vector {
        let rot = Rotation2::new(match self.itself.direction {
            Backward => PI,
            Forward => 0.0,
        });
        rot * self.segment.get_v()
    }
    fn get_pos(&self) -> (Pos, Pos) {
        let (segment_begin_pos, segment_end_pos) = self.segment.get_pos();
        let v_offset = {
            let lane_count_from_edge = match self.itself.direction {
                Backward => self.segment.itself.backward_lanes.len() - self.itself.rank - 1,
                Forward => self.segment.itself.backward_lanes.len() + self.itself.rank,
            };
            let v_ortho = self.segment.get_v_ortho();
            let v_segment_edge = (-0.5)
                * ROAD_LANE_WIDTH
                * (self.segment.itself.backward_lanes.len()
                    + self.segment.itself.forward_lanes.len()) as f64
                * v_ortho;
            let v_lane_edge =
                v_segment_edge + (lane_count_from_edge as f64 * ROAD_LANE_WIDTH * v_ortho);
            v_lane_edge + (0.5 * ROAD_LANE_WIDTH * v_ortho)
        };
        match self.itself.direction {
            Backward => (segment_end_pos + v_offset, segment_begin_pos + v_offset),
            Forward => (segment_begin_pos + v_offset, segment_end_pos + v_offset),
        }
    }
}
