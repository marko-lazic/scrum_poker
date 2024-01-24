use crate::channel::{RoomEvent, RoomMessage, RoomRequest, RoomResponse};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Participant {
    pub session_id: Uuid,
    pub name: Arc<str>,
    pub estimate: Arc<str>,
}

impl Participant {
    pub fn new(session_id: Uuid, name: Arc<str>) -> Participant {
        Participant {
            session_id,
            name,
            estimate: Arc::from(""),
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
    pub participants: Mutex<HashMap<Uuid, Participant>>,
}

impl Room {
    pub fn new(room_id: Arc<str>) -> Self {
        Room {
            room_id,
            show: false,
            participants: Mutex::new(HashMap::new()),
        }
    }

    pub async fn run(
        &self,
        mut room_rx: mpsc::Receiver<RoomMessage>,
        room_bc_tx: broadcast::Sender<RoomEvent>,
        ready_notifier: oneshot::Sender<()>,
    ) {
        tracing::info!("Room {} ready.", self.room_id);
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
        broadcast: &broadcast::Sender<RoomEvent>,
    ) {
        match request {
            RoomRequest::AddParticipant(p) => {
                self.join_participant(&p, resposne, broadcast).await;
            }
            RoomRequest::Estimate(e) => {
                tracing::trace!("Update estimate {:?}", e);
                resposne.send(RoomResponse::EstimateRecieved).unwrap();
                let mut participants = self.participants.lock().await;

                if let Some(participant) = participants.get_mut(&e.session_id) {
                    participant.estimate = e.value;
                    let _ = broadcast.send(RoomEvent::Update(participant.clone()));
                } else {
                    tracing::error!(
                        "Participant with session_id {} not found in room {}",
                        e.session_id,
                        self.room_id
                    );
                }
            }
        }
    }

    async fn join_participant(
        &self,
        p: &Participant,
        response: oneshot::Sender<RoomResponse>,
        broadcast: &broadcast::Sender<RoomEvent>,
    ) {
        let mut map = self.participants.lock().await;
        match map.get(&p.session_id) {
            Some(_existing_participant) => {}
            None => {
                map.insert(p.session_id, p.to_owned());
                let new_participant = map.get(&p.session_id).unwrap().to_owned();
                let _ = broadcast.send(RoomEvent::ParticipantJoined(new_participant));
            }
        };
        response
            .send(RoomResponse::ListParticipants(map.clone()))
            .unwrap();
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
