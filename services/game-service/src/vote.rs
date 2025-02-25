use common::prelude::*;
use tracing::info;

// Vote service function
pub fn vote_service(request: RpcRequest) -> Result<RpcResponse, RpcError> {
    // Parse the parameters
    let params: VoteRequest = request.parse_params().unwrap();

    // Process the vote (in a real app, this would interact with game state)
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
    Ok(RpcResponse::success_unchecked(response, request.id))
}
