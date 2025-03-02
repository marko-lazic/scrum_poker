use common::prelude::*;
use tracing::{info, warn};

use crate::dispatch::ServiceContext;

pub struct VoteCounter(pub usize);

// Vote service function
pub fn vote_service(ctx: ServiceContext) -> Result<RpcResponse, RpcError> {
    // Parse the parameters
    let params: VoteRequest = ctx.request.parse_params().unwrap();

    // Try to access VoteCounter resource
    if let Some(vote_counter) = ctx.resources.get::<VoteCounter>() {
        info!("Vote count is now {}", vote_counter.0);
    } else {
        warn!("VoteCounter resource not found");
    }

    // Process the vote
    info!(
        "Player {} voted for card {} in room {}",
        params.player_id, params.card, params.room_id
    );

    // Create response
    let response = VoteResponse {
        status: "success".to_string(),
        message: format!(
            "Vote recorded for player {} in room {}",
            params.player_id, params.room_id
        ),
    };

    // Return the response with the same ID as the request
    Ok(RpcResponse::success_unchecked(response, ctx.request.id))
}
