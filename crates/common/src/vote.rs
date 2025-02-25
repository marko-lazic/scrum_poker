use serde::{Deserialize, Serialize};

// Define the vote request and response structures
#[derive(Serialize, Deserialize, Debug)]
pub struct VoteRequest {
    pub player_id: String,
    pub room_id: String,
    pub card: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteResponse {
    pub status: String,
    pub message: String,
}
