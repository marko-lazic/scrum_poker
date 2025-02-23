use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CardRequest {
    pub player_id: String,
    pub room_id: String,
    pub event: String,
    pub card: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CardResponse {
    pub status: String,
    pub message: String,
}
