use std::{collections::HashMap, sync::Arc};

use tokio::sync::{broadcast, mpsc, oneshot, RwLock};

use crate::{
    channel::{RoomBroadcast, RoomChannel, RoomMessage, RoomRequest, RoomResponse},
    database,
    room::Room,
};

#[derive(Clone)]
pub struct AppState {
    pub addr: std::net::SocketAddr,
    pub pool: Arc<database::Pool>,
    pub view: dioxus_liveview::LiveViewPool,
    pub room_channels: Arc<RwLock<HashMap<String, RoomChannel>>>,
}

impl AppState {
    pub fn new() -> AppState {
        let mgr = database::Manager {};
        let pool = database::Pool::builder(mgr).max_size(50).build().unwrap();
        AppState {
            addr: ([127, 0, 0, 1], 3030).into(),
            pool: Arc::new(pool),
            view: dioxus_liveview::LiveViewPool::new(),
            room_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn spawn_or_find_room(&self, room_id: Arc<String>) -> RoomChannel {
        let channel = self.find_channel(room_id.clone()).await;
        if channel.is_some() {
            println!("User gets existing room channel");
            return channel.unwrap();
        } else {
            return self.spawn_room(room_id.clone()).await;
        }
    }

    async fn spawn_room(&self, room_id: Arc<String>) -> RoomChannel {
        let (ready_notifier, ready_receiver) = oneshot::channel();
        let (room_tx, room_rx) = self.create_mpsc_channel();
        let (room_bc_tx, _room_bc_rx) = self.create_broadcast_channel();
        let rid = room_id.clone();
        let rbctx = room_bc_tx.clone();
        tokio::spawn(async move {
            let room = Room::new(Arc::from(rid.as_str()));
            room.run(room_rx, rbctx, ready_notifier).await;
        });

        ready_receiver.await.ok();

        let mut w_rooms = self.room_channels.write().await;

        let new_room_ch = RoomChannel {
            tx: room_tx,
            bc_tx: room_bc_tx,
        };
        w_rooms.insert(room_id.to_string(), new_room_ch.clone());

        return new_room_ch;
    }

    async fn find_channel(&self, room_id: Arc<String>) -> Option<RoomChannel> {
        let r_rooms = self.room_channels.read().await;
        return r_rooms.get(&room_id.to_string()).cloned();
    }

    fn create_mpsc_channel(&self) -> (mpsc::Sender<RoomMessage>, mpsc::Receiver<RoomMessage>) {
        let (tx, rx) = mpsc::channel::<(RoomRequest, oneshot::Sender<RoomResponse>)>(10);
        return (tx, rx);
    }

    fn create_broadcast_channel(
        &self,
    ) -> (
        broadcast::Sender<RoomBroadcast>,
        broadcast::Receiver<RoomBroadcast>,
    ) {
        let (tx, rx) = broadcast::channel::<RoomBroadcast>(10);
        return (tx, rx);
    }
}
