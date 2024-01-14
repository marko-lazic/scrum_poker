use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    broadcast::{self, Sender},
    RwLock,
};

use crate::pool::{Manager, Pool};

#[derive(Clone)]
pub struct RoomChannel {
    pub tx: Sender<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub addr: std::net::SocketAddr,
    pub pool: Arc<Pool>,
    pub view: dioxus_liveview::LiveViewPool,
    pub room_channels: Arc<RwLock<HashMap<String, RoomChannel>>>,
}

impl AppState {
    pub fn new() -> AppState {
        let mgr = Manager {};
        let pool = Pool::builder(mgr).max_size(50).build().unwrap();
        AppState {
            addr: ([127, 0, 0, 1], 3030).into(),
            pool: Arc::new(pool),
            view: dioxus_liveview::LiveViewPool::new(),
            room_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn find_channel(&self, room_id: Arc<String>) -> Option<RoomChannel> {
        let r_rooms = self.room_channels.read().await;
        return r_rooms.get(&room_id.to_string()).cloned();
    }

    pub async fn create_channel(&self, room_id: Arc<String>) -> RoomChannel {
        let mut w_rooms = self.room_channels.write().await;
        let (tx, _) = broadcast::channel::<String>(10);
        let channel = RoomChannel { tx: tx.clone() };
        w_rooms.insert(room_id.to_string(), channel.clone());
        return channel;
    }
}
