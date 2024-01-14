use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

use tokio::sync::oneshot;

use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct Participant {
    pub session_id: Arc<String>,
    pub name: Arc<String>,
    pub estimate: Arc<String>,
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

    pub async fn run(self, tx: Sender<String>, ready_notifier: oneshot::Sender<()>) {
        let mut rx = tx.subscribe();
        println!("Created room task {}", self.room_id);
        let _ = ready_notifier.send(());

        loop {
            let msg = rx.recv().await;
            println!("Room received [{:?}]", msg);
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
