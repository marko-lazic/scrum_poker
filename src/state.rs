use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    broadcast::{self},
    oneshot, RwLock,
};

use crate::{
    channel::{RoomChannel, RoomMessage},
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

    pub async fn get_or_create_room_channel(&self, room_id: Arc<String>) -> RoomChannel {
        let channel = self.find_channel(room_id.clone()).await;
        if channel.is_some() {
            println!("User gets existing room channel");
            return channel.unwrap();
        } else {
            return self.spawn_room(room_id.clone()).await;
        }
    }

    pub async fn spawn_room(&self, room_id: Arc<String>) -> RoomChannel {
        let (ready_notifier, ready_receiver) = oneshot::channel();
        let channel = self.create_channel(room_id.clone()).await;
        let tx = channel.tx.clone();
        tokio::spawn(async move {
            let room = Room::new(room_id.to_string());
            room.run(tx, ready_notifier).await;
        });

        ready_receiver.await.ok();
        return channel;
    }

    pub async fn find_channel(&self, room_id: Arc<String>) -> Option<RoomChannel> {
        let r_rooms = self.room_channels.read().await;
        return r_rooms.get(&room_id.to_string()).cloned();
    }

    pub async fn create_channel(&self, room_id: Arc<String>) -> RoomChannel {
        let mut w_rooms = self.room_channels.write().await;
        let (tx, _) = broadcast::channel::<RoomMessage>(10);
        let channel = RoomChannel { tx: tx.clone() };
        w_rooms.insert(room_id.to_string(), channel.clone());
        return channel;
    }
}
