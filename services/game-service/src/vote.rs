use common::prelude::*;
use tracing::{info, warn};

use crate::service::ServiceContext;

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

#[cfg(test)]
mod tests {
    use common::prelude::{VoteRequest, VoteResponse};

    use crate::{
        app::App,
        vote::{VoteCounter, vote_service},
    };

    use super::*;

    #[test]
    fn start_app() {
        let mut app = App::new();
        app.add_service("vote", vote_service);
        app.insert_resource(VoteCounter(0));

        let params = VoteRequest {
            player_id: "player1".to_string(),
            room_id: "room1".to_string(),
            card: "1".to_string(),
        };
        let request = RpcRequest::new(
            "vote".to_string(),
            rmp_serde::to_vec(&params).unwrap(),
            None,
        );
        let rpc_response = app.run(request);
        let vote_response = rpc_response.parse_result::<VoteResponse>().unwrap();
        assert_eq!(vote_response.status, "success".to_string());
    }
}
