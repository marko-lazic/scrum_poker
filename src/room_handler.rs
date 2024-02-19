use std::{collections::HashMap, sync::Arc};

use tokio::sync::{broadcast, mpsc, oneshot, RwLock};

use crate::{
    channel::{
        CtrlMessage, RoomBroadcastMessage, RoomChannel, RoomMessage, RoomRequest, RoomResponse,
    },
    room::Room,
};

#[derive(Debug)]
pub enum CtrlRequest {
    HealthCheck,
}

#[derive(Debug)]
pub enum CtrlResponse {
    Health(HealthStatus),
}

#[derive(Debug)]
pub enum HealthStatus {
    Healthy,
}

#[derive(Clone)]
pub struct RoomHandler {
    pub room_channels: Arc<RwLock<HashMap<String, RoomChannel>>>,
}

impl RoomHandler {
    pub fn new() -> RoomHandler {
        RoomHandler {
            room_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn spawn_or_find_room(&self, room_id: Arc<str>) -> RoomChannel {
        let channel = self.find_channel(room_id.clone()).await;
        if channel.is_some() {
            return channel.unwrap();
        } else {
            return self.spawn_room(room_id.clone()).await;
        }
    }

    async fn find_channel(&self, room_id: Arc<str>) -> Option<RoomChannel> {
        let r_rooms = self.room_channels.read().await;
        return r_rooms.get(&room_id.to_string()).cloned();
    }

    async fn spawn_room(&self, room_id: Arc<str>) -> RoomChannel {
        let (room_tx, room_rx) = self.create_mpsc_channel();
        let (room_bc_tx, _room_bc_rx) = self.create_broadcast_channel();
        let rid = room_id.clone();
        let new_room_ch = RoomChannel {
            tx: room_tx,
            broadcast: room_bc_tx,
        };
        let channel = new_room_ch.clone();

        let (ctx, crx) = mpsc::channel::<CtrlMessage>(10);

        tokio::spawn(async move {
            let room = Room::new(rid, channel);
            room.run(room_rx, crx).await;
        });

        let (htx, hrx) = oneshot::channel::<CtrlResponse>();
        _ = ctx.send((CtrlRequest::HealthCheck, htx)).await;
        hrx.await.ok();

        let mut w_rooms = self.room_channels.write().await;

        w_rooms.insert(room_id.to_string(), new_room_ch.clone());

        return new_room_ch;
    }

    fn create_mpsc_channel(&self) -> (mpsc::Sender<RoomMessage>, mpsc::Receiver<RoomMessage>) {
        let (tx, rx) = mpsc::channel::<(RoomRequest, oneshot::Sender<RoomResponse>)>(10);
        return (tx, rx);
    }

    fn create_broadcast_channel(
        &self,
    ) -> (
        broadcast::Sender<RoomBroadcastMessage>,
        broadcast::Receiver<RoomBroadcastMessage>,
    ) {
        let (tx, rx) = broadcast::channel::<RoomBroadcastMessage>(10);
        return (tx, rx);
    }
}
