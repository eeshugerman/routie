use crate::{actor, road, util::CloneEmpty};

pub fn advance(network_past: road::Network) -> road::Network {
    let mut network_future = network_past.clone_empty();
    for (id, segment) in network_past.segments.enumerate() {
        let segment_ctx = &road::SegmentContext::new(&network_past, id, segment);
        for (pos_param, actor) in segment.backward_actors.enumerate() {
            let actor_ctx = actor::ActorContext::OffRoad {
                pos_param: *pos_param,
                segment_ctx,
                segment_side: road::Direction::Backward,
                actor,
            };
            actor_ctx.advance(&mut network_future);
        }
        for (pos_param, actor) in segment.forward_actors.enumerate() {
            let actor_ctx = actor::ActorContext::OffRoad {
                pos_param: *pos_param,
                segment_ctx,
                segment_side: road::Direction::Forward,
                actor,
            };
            actor_ctx.advance(&mut network_future);
        }
        for (rank, lane) in segment.backward_lanes.enumerate() {
            let lane_ctx =
                &road::SegmentLaneContext::new(&segment_ctx, road::Direction::Backward, rank, lane);
            for (pos_param, actor) in lane.actors.enumerate() {
                let actor_ctx =
                    actor::ActorContext::OnRoadSegment { pos_param: *pos_param, lane_ctx, actor };
                actor_ctx.advance(&mut network_future);
            }
        }
        for (rank, lane) in segment_ctx.segment.forward_lanes.enumerate() {
            let lane_ctx =
                &road::SegmentLaneContext::new(&segment_ctx, road::Direction::Forward, rank, lane);
            for (pos_param, actor) in lane.actors.enumerate() {
                let actor_ctx =
                    actor::ActorContext::OnRoadSegment { pos_param: *pos_param, lane_ctx, actor };
                actor_ctx.advance(&mut network_future);
            }
        }
    }
    for (id, junction) in network_past.junctions.enumerate() {
        // TODO
    }
    network_future
}
