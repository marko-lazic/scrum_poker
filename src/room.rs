use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::Arc,
};

use tokio::sync::{mpsc, oneshot, Mutex};

use uuid::Uuid;

use crate::channel::{RoomMessage, RoomRequest, RoomResponse};

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
    pub room_id: Arc<String>,
    pub show: bool,
    pub participants: Mutex<HashSet<Participant>>,
}

impl Room {
    pub fn new(room_id: String) -> Self {
        Room {
            room_id: Arc::from(room_id),
            show: false,
            participants: Mutex::new(HashSet::new()),
        }
    }

    pub async fn run(
        &self,
        mut rx: mpsc::Receiver<RoomMessage>,
        ready_notifier: oneshot::Sender<()>,
    ) {
        println!("Room task {} is ready!", self.room_id);
        let _ = ready_notifier.send(());

        loop {
            while let Some((request, response)) = rx.recv().await {
                self.update_room(request).await;
                response.send(RoomResponse::Ok).unwrap();
            }
        }
    }

    async fn update_room(&self, request: RoomRequest) {
        match request {
            RoomRequest::AddParticipant(p) => self.insert_participant(p).await,
            RoomRequest::Estimate(e) => println!("Estimate {:?}", e),
        }
    }

    async fn insert_participant(&self, p: Participant) {
        if self.participants.lock().await.insert(p.clone()) {
            println!("Participant {} inserted successfully!", p.name);
        } else {
            println!("Participant {} already exists in the set.", p.name);
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
