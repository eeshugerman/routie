use crate::{actor::ActorContext, road, util::CloneEmpty};

pub fn advance(network_last: road::Network) -> road::Network {
    let network_next = network_last.clone_empty();
    for (id, segment) in network_last.segments.enumerate() {
        for (rank, lane) in segment.forward_lanes.enumerate() {
            for (pos_param, _) in lane.actors.enumerate() {
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
