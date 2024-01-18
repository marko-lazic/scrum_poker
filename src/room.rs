use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::Arc,
};

use tokio::sync::{broadcast, mpsc, oneshot, Mutex};

use uuid::Uuid;

use crate::channel::{RoomEvent, RoomMessage, RoomRequest, RoomResponse};

#[derive(Debug, Clone)]
pub struct Participant {
    pub session_id: Uuid,
    pub name: Arc<String>,
    pub estimate: Arc<String>,
}

impl Participant {
    pub fn new(session_id: Uuid, name: Arc<String>) -> Participant {
        Participant {
            session_id,
            name,
            estimate: Arc::new("".to_string()),
        }
    }
}

impl Hash for Participant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.session_id.hash(state);
    }
}

impl PartialEq for Participant {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
    }
}

impl Eq for Participant {}

#[derive(Debug)]
pub struct Room {
    pub room_id: Arc<str>,
    pub show: bool,
    pub participants: Mutex<HashSet<Participant>>,
}

impl Room {
    pub fn new(room_id: Arc<str>) -> Self {
        Room {
            room_id,
            show: false,
            participants: Mutex::new(HashSet::new()),
        }
    }

    pub async fn run(
        &self,
        mut room_rx: mpsc::Receiver<RoomMessage>,
        room_bc_tx: broadcast::Sender<RoomEvent>,
        ready_notifier: oneshot::Sender<()>,
    ) {
        println!("Room {} ready.", self.room_id);
        let _ = ready_notifier.send(());

        loop {
            while let Some((request, response)) = room_rx.recv().await {
                self.update_room(request, response, &room_bc_tx).await;
            }
        }
    }

    async fn update_room(
        &self,
        request: RoomRequest,
        resposne: oneshot::Sender<RoomResponse>,
        tx: &broadcast::Sender<RoomEvent>,
    ) {
        match request {
            RoomRequest::AddParticipant(p) => {
                self.insert_participant(p.clone()).await;
                resposne
                    .send(RoomResponse::ListParticipants(
                        self.participants.lock().await.clone(),
                    ))
                    .unwrap();
                let _result = tx.send(RoomEvent::ParticipantJoined(p));
            }
            RoomRequest::Estimate(e) => {
                println!("Estimate {:?}", e);
                resposne.send(RoomResponse::EstimateRecieved).unwrap();
            }
        }
    }

    async fn insert_participant(&self, p: Participant) {
        self.participants.lock().await.replace(p.clone());
    }
}

impl Hash for Room {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.room_id.hash(state);
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.room_id == other.room_id
    }
}

impl Eq for Room {}
