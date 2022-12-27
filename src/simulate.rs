use crate::{
    actor,
    road,
    util::CloneEmpty,
};

pub fn advance(network_last: road::Network) -> road::Network {
    let mut network_next = network_last.clone_empty();
    for (id, segment) in network_last.segments.enumerate() {
        let segment_ctx = road::SegmentContext::new(&network_last, id, segment);
        for (rank, lane) in segment.forward_lanes.enumerate() {
            let lane_ctx = road::SegmentLaneContext::new(&segment_ctx, road::Direction::Forward, rank, lane);
            for (pos_param, actor) in lane.actors.enumerate() {
                let actor_ctx = actor::ActorContext::OnRoadSegment {
                    pos_param: *pos_param,
                    lane_ctx: &lane_ctx,
                    actor,
                };
                actor_ctx.advance(&mut network_next);
            }
        }
        // for (rank, lane) in segment_ctx.segment.backward_lanes.enumerate() {
        //     for (pos_param, actor) in lane_ctx.lane.actors.enumerate() {
        //     }
        // }
    }
    // for (id, junction) in self.road_network.junctions.enumerate() {
    // }
    network_next
}
