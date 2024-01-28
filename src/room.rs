use crate::channel::{RoomChannel, RoomEvent, RoomMessage, RoomRequest, RoomResponse};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::sync::{mpsc, oneshot, Mutex};
use uuid::Uuid;

#[derive(PartialEq, Debug, Clone)]
pub enum ParticipantStatus {
    Online,
    Left,
}

#[derive(Debug, Clone)]
pub struct Participant {
    pub session_id: Uuid,
    pub name: Arc<str>,
    pub estimate: Arc<str>,
    pub status: ParticipantStatus,
}

impl Participant {
    pub fn new(session_id: Uuid, name: Arc<str>) -> Participant {
        Participant {
            session_id,
            name,
            estimate: Arc::from(""),
            status: ParticipantStatus::Online,
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
    pub channel: RoomChannel,
    pub show: bool,
    pub participants: Mutex<HashMap<Uuid, Participant>>,
}

impl Room {
    pub fn new(room_id: Arc<str>, channel: RoomChannel) -> Self {
        Room {
            room_id,
            channel,
            show: false,
            participants: Mutex::new(HashMap::new()),
        }
    }

    pub async fn run(
        &self,
        mut room_rx: mpsc::Receiver<RoomMessage>,
        ready_notifier: oneshot::Sender<()>,
    ) {
        tracing::trace!("Room {} ready.", self.room_id);
        _ = ready_notifier.send(());

        loop {
            while let Some((request, response)) = room_rx.recv().await {
                self.update_room(request, response).await;
            }
        }
    }

    async fn update_room(&self, request: RoomRequest, response: oneshot::Sender<RoomResponse>) {
        match request {
            RoomRequest::Join(p) => {
                self.join_participant(p, response).await;
            }
            RoomRequest::Leave(session_id) => {
                self.leave_participant(session_id).await;
            }
            RoomRequest::Remove(session_id) => {
                self.remove_participant(session_id).await;
            }
            RoomRequest::Estimate(e) => {
                tracing::trace!("Update estimate {:?}", e);
                response.send(RoomResponse::EstimateRecieved).unwrap();
                let mut participants = self.participants.lock().await;

                if let Some(participant) = participants.get_mut(&e.session_id) {
                    participant.estimate = e.value;
                    _ = self
                        .channel
                        .broadcast
                        .send(RoomEvent::Update(participant.clone()));
                } else {
                    tracing::error!(
                        "Participant with session_id {} not found in room {}",
                        e.session_id,
                        self.room_id
                    );
                }
            }
            RoomRequest::Heartbeat(session_id) => {
                self.heartbeat_participant(session_id).await;
            }
        }
    }

    async fn join_participant(&self, p: Participant, response: oneshot::Sender<RoomResponse>) {
        let mut map = self.participants.lock().await;
        match map.get_mut(&p.session_id) {
            Some(existing_participant) => {
                existing_participant.status = ParticipantStatus::Online;
            }
            None => {
                map.insert(p.session_id, p.to_owned());
                let new_participant = map.get(&p.session_id).unwrap().to_owned();
                _ = self
                    .channel
                    .broadcast
                    .send(RoomEvent::Joined(new_participant));
            }
        };
        response
            .send(RoomResponse::ListParticipants(map.clone()))
            .unwrap();
    }

    async fn leave_participant(&self, session_id: Uuid) {
        let mut map = self.participants.lock().await;
        match map.get_mut(&session_id) {
            Some(participant) => {
                participant.status = ParticipantStatus::Left;
                self.spawn_cleanup_participant(session_id);
            }
            None => {}
        }
    }

    fn spawn_cleanup_participant(&self, session_id: Uuid) {
        let channel = self.channel.clone();
        tokio::task::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            _ = channel.broadcast.send(RoomEvent::AskForHeartbeat);
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            _ = channel.send(RoomRequest::Remove(session_id)).await;
        });
    }

    async fn remove_participant(&self, session_id: Uuid) {
        let mut map = self.participants.lock().await;
        match map.get_mut(&session_id) {
            Some(participant) => {
                tracing::trace!("Participant {} status {:?}", session_id, participant.status);
                if participant.status == ParticipantStatus::Left {
                    map.remove(&session_id);
                    tracing::trace!("Pemoving participant");
                    _ = self.channel.broadcast.send(RoomEvent::Left(session_id));
                }
            }
            None => {
                tracing::trace!("Not found session_id {}", session_id);
            }
        }
        tracing::trace!("Number of participants {}", map.len());
    }

    async fn heartbeat_participant(&self, session_id: Uuid) {
        let mut map = self.participants.lock().await;
        match map.get_mut(&session_id) {
            Some(participant) => {
                if participant.status == ParticipantStatus::Left {
                    participant.status = ParticipantStatus::Online;
                }
            }
            None => {}
        }
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
