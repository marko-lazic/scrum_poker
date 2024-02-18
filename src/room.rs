use crate::{
    channel::{EstimateVisibility, RoomChannel, RoomEvent, RoomMessage, RoomRequest, RoomResponse},
    estimate::Estimate,
};
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
    pub estimate: Estimate,
    pub status: ParticipantStatus,
}

impl Participant {
    pub fn new(session_id: Uuid, name: Arc<str>) -> Participant {
        Participant {
            session_id,
            name,
            estimate: Estimate::None,
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
    pub visibility: Mutex<EstimateVisibility>,
    pub participants: Mutex<HashMap<Uuid, Participant>>,
}

impl Room {
    pub fn new(room_id: Arc<str>, channel: RoomChannel) -> Self {
        Room {
            room_id,
            channel,
            visibility: Mutex::new(EstimateVisibility::Hidden),
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
            RoomRequest::SendEstimate(session_id, estimate_point) => {
                tracing::trace!(
                    "Update estimate session_id: {:?}, estimate: {}",
                    session_id,
                    estimate_point
                );
                let mut participants = self.participants.lock().await;

                if let Some(participant) = participants.get_mut(&session_id) {
                    participant.estimate = estimate_point;
                    _ = self
                        .channel
                        .broadcast
                        .send(RoomEvent::ParticipantUpdate(participant.clone()));
                } else {
                    tracing::error!(
                        "Update estimate: Participant with session_id {} not found in room {}",
                        session_id,
                        self.room_id
                    );
                }
            }
            RoomRequest::ChangeVisibility => {
                let mut visibility = self.visibility.lock().await;
                *visibility = visibility.toggle();
                _ = self
                    .channel
                    .broadcast
                    .send(RoomEvent::ChangedVisibility(visibility.clone()));
            }
            RoomRequest::DeleteEstimates => {
                self.delete_estimates().await;
                let mut visibility = self.visibility.lock().await;
                *visibility = EstimateVisibility::Hidden;

                _ = self.channel.broadcast.send(RoomEvent::EstimatesDeleted);
            }
            RoomRequest::Heartbeat(session_id) => {
                self.heartbeat_participant(session_id).await;
            }
            RoomRequest::NameChange(session_id, new_username) => {
                self.change_participant_name(session_id, new_username).await;
            }
        }
    }

    async fn change_participant_name(&self, session_id: Uuid, new_username: Arc<str>) {
        let mut map = self.participants.lock().await;
        match map.get_mut(&session_id) {
            Some(participant) => {
                participant.name = new_username;
                _ = self
                    .channel
                    .broadcast
                    .send(RoomEvent::ParticipantUpdate(participant.clone()));
            }
            None => {
                tracing::warn!("Tried to change participant username but not found in room");
            }
        }
    }

    async fn join_participant(&self, p: Participant, response: oneshot::Sender<RoomResponse>) {
        let mut map = self.participants.lock().await;
        match map.get_mut(&p.session_id) {
            Some(existing_participant) => {
                existing_participant.status = ParticipantStatus::Online;
                existing_participant.name = p.name;
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
        let room_visibility = self.visibility.lock().await.clone();
        response
            .send(RoomResponse::RoomState(map.clone(), room_visibility))
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
            _ = channel.broadcast.send(RoomEvent::RoomRequestedHeartbeat);
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

    async fn delete_estimates(&self) {
        let mut map = self.participants.lock().await;
        for (_, p) in map.iter_mut() {
            p.estimate = Estimate::None;
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
