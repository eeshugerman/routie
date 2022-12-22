use crate::{actor::ActorContext, road};

pub fn advance(road_network: &mut road::Network) {
    for (id, segment) in road_network.segments.enumerate() {
        let segment_ctx = &road::SegmentContext::new(road_network, id, segment);
        for (rank, lane) in segment_ctx.segment.forward_lanes.enumerate() {
            let lane_ctx =
                &road::SegmentLaneContext::new(segment_ctx, road::Direction::Forward, rank, &mut lane);
            for (pos_param, actor) in lane_ctx.lane.actors.enumerate() {
                let actor_ctx =
                    ActorContext::OnRoadSegment { pos_param: *pos_param, lane_ctx, actor };
            }
        }
        for (rank, lane) in segment_ctx.segment.backward_lanes.enumerate() {
            let lane_ctx =
                &road::SegmentLaneContext::new(segment_ctx, road::Direction::Backward, rank, &mut lane);
            for (pos_param, actor) in lane_ctx.lane.actors.enumerate() {
                let actor_ctx =
                    ActorContext::OnRoadSegment { pos_param: *pos_param, lane_ctx, actor };
            }
        }
    }
    // for (id, junction) in self.road_network.junctions.enumerate() {
    //     let junction_ctx = &road::JunctionContext::new(self.road_network, id, junction);
    // }
}
