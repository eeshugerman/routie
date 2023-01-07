use std::f64::consts::{FRAC_PI_2, PI};

use lyon_geom::QuadraticBezierSegment;
// TODO: use lyon_geom stuff instead
use nalgebra::{Point2, Rotation2, Vector2};

use crate::{
    actor,
    constants::{ROAD_JUNCTION_RADIUS, ROAD_LANE_WIDTH, ROAD_SEGMENT_WIGGLE_ROOM_PCT},
    road::{
        self,
        Direction::{Backward, Forward},
        QualifiedSegmentLaneRank, SegmentContext, SegmentLaneContext,
    },
};

pub type Pos = Point2<f64>;
pub type Vector = Vector2<f64>;

pub trait PointLike {
    fn get_pos(&self) -> Pos;
}

impl<'a> PointLike for road::JunctionContext<'a> {
    fn get_pos(&self) -> Pos {
        self.junction.pos
    }
}

impl<'a> PointLike for actor::ActorContext<'a> {
    fn get_pos(&self) -> Pos {
        match self {
            actor::ActorContext::OffRoad { pos_param, segment_ctx, segment_side, actor } => {
                let (segment_begin_pos, _) = segment_ctx.get_pos();
                let (offset_direction, scalar) = match segment_side {
                    road::Direction::Forward => (1.0, *pos_param),
                    road::Direction::Backward => (-1.0, 1.0 - *pos_param),
                };
                let offset = offset_direction * segment_ctx.get_width() * segment_ctx.get_v_ortho();
                segment_begin_pos + (scalar * segment_ctx.get_v()) + offset
            }
            actor::ActorContext::OnRoadSegment { pos_param, lane_ctx, actor } => {
                let (lane_begin_pos, _) = lane_ctx.get_pos();
                lane_begin_pos + *pos_param * lane_ctx.get_v()
            }
            actor::ActorContext::OnRoadJunction { pos_param, lane_ctx, actor } => {
                let (curve_start_pos, _) = lane_ctx.get_pos();
                let curve = lane_ctx.get_curve().sample(*pos_param);
                curve_start_pos + Vector2::new(curve.x, curve.y)
            },
        }
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
        rot * self.get_v_norm()
    }
}

impl<'a> LineLike for road::SegmentContext<'a> {
    fn get_width(&self) -> f64 {
        let total_lane_count = self.segment.forward_lanes.len() + self.segment.backward_lanes.len();
        (1.0 + (ROAD_SEGMENT_WIGGLE_ROOM_PCT as f64 / 100.0))
            * ROAD_LANE_WIDTH
            * std::cmp::max(total_lane_count, 1) as f64
    }

    fn get_v_norm(&self) -> Vector {
        let (begin_junction_ctx, end_junction_ctx) = self.get_junctions();
        (end_junction_ctx.junction.pos - begin_junction_ctx.junction.pos).normalize()
    }

    fn get_pos(&self) -> (Pos, Pos) {
        let (begin_junction_ctx, end_junction_ctx) = self.get_junctions();
        (
            begin_junction_ctx.junction.pos + (ROAD_JUNCTION_RADIUS * self.get_v_norm()),
            end_junction_ctx.junction.pos + (-1.0 * ROAD_JUNCTION_RADIUS * self.get_v_norm()),
        )
    }
}

impl<'a> LineLike for road::SegmentLaneContext<'a> {
    fn get_width(&self) -> f64 {
        ROAD_LANE_WIDTH
    }
    fn get_v(&self) -> Vector {
        let rot = Rotation2::new(match self.lane.direction {
            Backward => PI,
            Forward => 0.0,
        });
        rot * self.segment_ctx.get_v()
    }
    fn get_pos(&self) -> (Pos, Pos) {
        let (segment_begin_pos, segment_end_pos) = self.segment_ctx.get_pos();
        let v_lat_offset = {
            let rank: i32 = self.rank.into();
            let lane_count_from_edge = match self.lane.direction {
                Backward => self.segment_ctx.segment.backward_lanes.len() as i32 - rank - 1,
                Forward => self.segment_ctx.segment.backward_lanes.len() as i32 + rank,
            };
            let v_ortho = self.segment_ctx.get_v_ortho();
            let v_segment_edge = (-0.5)
                * ROAD_LANE_WIDTH
                * (self.segment_ctx.segment.backward_lanes.len()
                    + self.segment_ctx.segment.forward_lanes.len()) as f64
                * v_ortho;
            let v_lane_edge =
                v_segment_edge + (lane_count_from_edge as f64 * ROAD_LANE_WIDTH * v_ortho);
            v_lane_edge + (0.5 * ROAD_LANE_WIDTH * v_ortho)
        };
        match self.lane.direction {
            Backward => (segment_end_pos + v_lat_offset, segment_begin_pos + v_lat_offset),
            Forward => (segment_begin_pos + v_lat_offset, segment_end_pos + v_lat_offset),
        }
    }
}

impl<'a> road::JunctionLaneContext<'a> {
    pub fn get_pos(&self) -> (Pos, Pos) {
        let (input_segment_lane, output_segment_lane) =
            self.junction.get_segment_lanes_for_junction_lane(self.id);

        let to_pos = |(segment_id, direction, rank): QualifiedSegmentLaneRank| {
            let segment = self.junction.network.segments.get(&segment_id).unwrap();
            let segment_ctx = SegmentContext::new(self.junction.network, segment_id, segment);
            let segment_lane = segment.get_lanes(direction).get(&rank).unwrap();
            SegmentLaneContext::new(&segment_ctx, direction, rank, segment_lane).get_pos()
        };
        let (_, input_end_pos) = to_pos(input_segment_lane);
        let (output_begin_pos, _) = to_pos(output_segment_lane);
        (input_end_pos, output_begin_pos)
    }

    // TODO: memoize
    pub fn get_curve(&self) -> QuadraticBezierSegment<f64> {
        let to_lyon_point = |p: Pos| lyon_geom::Point::new(p.x, p.y);
        let to_lyon_vector = |v: Vector| lyon_geom::Vector::new(v.x, v.y);
        let to_line = |(segment_id, direction, rank): QualifiedSegmentLaneRank| {
            let segment = self.junction.network.segments.get(&segment_id).unwrap();
            let segment_ctx = SegmentContext::new(self.junction.network, segment_id, segment);
            let segment_lane = segment_ctx.segment.get_lanes(direction).get(&rank).unwrap();
            let segment_lane_ctx =
                SegmentLaneContext::new(&segment_ctx, direction, rank, segment_lane);
            lyon_geom::Line {
                point: to_lyon_point(segment_lane_ctx.get_pos().0),
                vector: to_lyon_vector(segment_lane_ctx.get_v()),
            }
        };

        let (begin_pos, end_pos) = self.get_pos();
        let (input_segment_lane, output_segment_lane) =
            self.junction.get_segment_lanes_for_junction_lane(self.id);
        let input_lane_line = to_line(input_segment_lane);
        let output_lane_line = to_line(output_segment_lane);
        let intersect_pos = match input_lane_line.intersection(&output_lane_line) {
            None => begin_pos + 0.5 * (end_pos - begin_pos),
            Some(p) => Pos::new(p.x, p.y),
        };
        QuadraticBezierSegment {
            from: to_lyon_point(begin_pos),
            ctrl: to_lyon_point(intersect_pos),
            to: to_lyon_point(end_pos),
        }
    }
}
