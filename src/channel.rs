use std::sync::Arc;

use dioxus::{
    core::ScopeState,
    hooks::{use_shared_state, UseSharedState},
};
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

use crate::room::Participant;

#[derive(Clone, Debug)]
pub enum RoomRequest {
    AddParticipant(Participant),
    Estimate(EstimateData),
}

#[derive(Clone, Debug)]
pub enum RoomResponse {
    Ok,
}

#[derive(Clone, Debug)]
pub enum RoomBroadcast {
    State,
}

#[derive(Clone, Debug)]
pub struct EstimateData {
    pub session_id: Uuid,
    pub value: Arc<str>,
}

pub type RoomMessage = (RoomRequest, oneshot::Sender<RoomResponse>);

#[derive(Clone)]
pub struct RoomChannel {
    pub tx: mpsc::Sender<RoomMessage>,
    pub bc_tx: broadcast::Sender<RoomBroadcast>,
}

impl RoomChannel {
    pub async fn send(&self, msg: RoomRequest) -> RoomResponse {
        let tx = self.tx.clone();
        let result = tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();

            match tx.send((msg, resp_tx)).await {
                Ok(_) => {
                    // println!("Message sent successfully");
                }
                Err(err) => {
                    println!("Error sending message: {}", err);
                }
            }

            return resp_rx.await.unwrap();
        });
        return result.await.unwrap();
    }

    // pub fn subscribe(&self) -> broadcast::Receiver<(RoomMessage, oneshot::Sender<RoomResponse>)> {
    //     return self.tx.subscribe();
    // }
}

pub fn use_room_channel(cx: &ScopeState) -> &UseSharedState<RoomChannel> {
    use_shared_state::<RoomChannel>(cx).expect("Room Channel not provided")
}
