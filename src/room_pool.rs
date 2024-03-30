use std::{collections::HashMap, sync::Arc};

use tokio::sync::{broadcast, mpsc, oneshot, RwLock};

const BUFFER_SIZE: usize = 256;

use crate::{
    channel::{RoomBroadcastMessage, RoomChannel, RoomMessage, RoomRequest, RoomResponse},
    error::ScError,
    room::{Room, RoomId},
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

pub type CtrlMessage = (CtrlRequest, oneshot::Sender<CtrlResponse>);

#[derive(Debug)]
pub enum RoomPoolRequest {
    Spawn(RoomId),
    Shutdown(RoomId),
}

#[derive(Debug)]
pub enum RoomPoolResponse {
    Channel(RoomChannel),
    GrantShutdown,
}

pub type RoomPoolMessage = (RoomPoolRequest, oneshot::Sender<RoomPoolResponse>);

#[derive(Clone)]
pub struct RoomPoolChannel {
    pub request_tx: mpsc::Sender<RoomPoolMessage>,
}

impl RoomPoolChannel {
    pub async fn spawn(&self, room_id: &RoomId) -> Result<RoomChannel, ScError> {
        let response = self.send(RoomPoolRequest::Spawn(room_id.clone())).await?;
        match response {
            RoomPoolResponse::Channel(ch) => Ok(ch),
            _ => Err(ScError::UnexpectedResponse),
        }
    }

    pub async fn shutdown(&self, room_id: &RoomId) -> Result<(), ScError> {
        self.send(RoomPoolRequest::Shutdown(room_id.clone()))
            .await?;
        Ok(())
    }

    async fn send(&self, msg: RoomPoolRequest) -> Result<RoomPoolResponse, ScError> {
        let (rp_tx, rpr_rx) = oneshot::channel::<RoomPoolResponse>();
        let rp_message: RoomPoolMessage = (msg, rp_tx);
        match self.request_tx.send(rp_message).await {
            Ok(_) => match rpr_rx.await {
                Ok(response) => Ok(response),
                Err(err) => Err(ScError::OneshotRecieveError(err)),
            },
            Err(err) => {
                tracing::info!("Error sending message: {}", err);
                Err(ScError::RoomPoolMessageSendError(err))
            }
        }
    }
}

pub struct RoomPool {
    room_channels: Arc<RwLock<HashMap<RoomId, RoomChannel>>>,
    room_pool_channel: RoomPoolChannel,
    room_pool_rx: mpsc::Receiver<RoomPoolMessage>,
}

impl RoomPool {
    pub fn spawn() -> RoomPoolChannel {
        let (retx, rerx) = mpsc::channel::<RoomPoolMessage>(BUFFER_SIZE);

        let room_pool_chanel = RoomPoolChannel { request_tx: retx };

        let rp_ch = room_pool_chanel.clone();
        tokio::spawn(async move {
            let mut room_pool = RoomPool {
                room_channels: Arc::new(RwLock::new(HashMap::new())),
                room_pool_channel: rp_ch,
                room_pool_rx: rerx,
            };
            room_pool.spawn_room_pool().await;
        });

        room_pool_chanel
    }

    async fn spawn_room_pool(&mut self) {
        loop {
            while let Some((request, response)) = self.room_pool_rx.recv().await {
                match request {
                    RoomPoolRequest::Spawn(room_id) => {
                        let room_ch = self.spawn_or_find_room(room_id).await;
                        response.send(RoomPoolResponse::Channel(room_ch)).unwrap();
                    }
                    RoomPoolRequest::Shutdown(room_id) => {
                        let mut map = self.room_channels.write().await;
                        map.remove(&room_id);
                        response.send(RoomPoolResponse::GrantShutdown).unwrap();
                        tracing::trace!("Granted shutdown for room_id: {}", room_id);
                    }
                }
            }
        }
    }

    pub async fn spawn_or_find_room(&self, room_id: RoomId) -> RoomChannel {
        if let Some(channel) = self.find_channel(room_id.clone()).await {
            channel
        } else {
            self.spawn_room(room_id.clone()).await
        }
    }

    async fn find_channel(&self, room_id: RoomId) -> Option<RoomChannel> {
        let r_rooms = self.room_channels.read().await;
        return r_rooms.get(&room_id).cloned();
    }

    async fn spawn_room(&self, room_id: RoomId) -> RoomChannel {
        let (room_tx, room_rx) = self.create_room_request_sender_channel();
        let (room_bc_tx, _room_bc_rx) = self.create_room_broadcast_channel();
        let rid = room_id.clone();
        let new_room_ch = RoomChannel {
            tx: room_tx,
            broadcast: room_bc_tx,
        };
        let channel = new_room_ch.clone();

        let (ctrl_tx, ctrl_rx) = mpsc::channel::<CtrlMessage>(BUFFER_SIZE);

        let room_pool_ch = self.room_pool_channel.clone();
        tokio::spawn(async move {
            let room = Room::new(rid, channel);
            room.run(room_rx, ctrl_rx, room_pool_ch).await;
        });

        let (htx, hrx) = oneshot::channel::<CtrlResponse>();
        _ = ctrl_tx.send((CtrlRequest::HealthCheck, htx)).await;
        hrx.await.ok();

        let mut w_rooms = self.room_channels.write().await;

        w_rooms.insert(room_id, new_room_ch.clone());

        new_room_ch
    }

    fn create_room_request_sender_channel(
        &self,
    ) -> (mpsc::Sender<RoomMessage>, mpsc::Receiver<RoomMessage>) {
        mpsc::channel::<(RoomRequest, oneshot::Sender<RoomResponse>)>(BUFFER_SIZE)
    }

    fn create_room_broadcast_channel(
        &self,
    ) -> (
        broadcast::Sender<RoomBroadcastMessage>,
        broadcast::Receiver<RoomBroadcastMessage>,
    ) {
        broadcast::channel::<RoomBroadcastMessage>(BUFFER_SIZE)
    }
}
