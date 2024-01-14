use std::sync::Arc;

use dioxus::{
    core::ScopeState,
    hooks::{use_shared_state, UseSharedState},
};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::room::Participant;

#[derive(Clone, Debug)]
pub enum RoomMessage {
    AddParticipant(Participant),
    Estimate(EstimateData),
}

#[derive(Clone, Debug)]
pub struct EstimateData {
    pub session_id: Uuid,
    pub value: Arc<str>,
}

#[derive(Clone)]
pub struct RoomChannel {
    pub tx: broadcast::Sender<RoomMessage>,
}

impl RoomChannel {
    pub fn send(&self, msg: RoomMessage) {
        match self.tx.send(msg) {
            Ok(_) => {
                // println!("Message sent successfully");
            }
            Err(err) => {
                println!("Error sending message: {}", err);
            }
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RoomMessage> {
        return self.tx.subscribe();
    }
}

pub fn use_room_channel(cx: &ScopeState) -> &UseSharedState<RoomChannel> {
    use_shared_state::<RoomChannel>(cx).expect("Room Channel not provided")
}
