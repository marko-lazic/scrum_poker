use crate::{error::ScError, room::Participant};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum RoomRequest {
    AddParticipant(Participant),
    Estimate(EstimateData),
}

#[derive(Clone, Debug)]
pub enum RoomResponse {
    ListParticipants(HashMap<Uuid, Participant>),
    EstimateRecieved,
}

#[derive(Clone, Debug)]
pub enum RoomEvent {
    ParticipantJoined(Participant),
    Update(Participant),
}

#[derive(Clone, Debug)]
pub struct EstimateData {
    pub session_id: Uuid,
    pub value: Arc<str>,
}

pub type RoomMessage = (RoomRequest, oneshot::Sender<RoomResponse>);

#[derive(Clone)]
pub struct RoomChannel {
    pub tx: mpsc::Sender<RoomMessage>,
    pub bc_tx: broadcast::Sender<RoomEvent>,
}

impl RoomChannel {
    pub async fn send(&self, msg: RoomRequest) -> Result<RoomResponse, ScError> {
        let handle: tokio::task::JoinHandle<Result<RoomResponse, ScError>> =
            self.spawn_send(msg).await;

        let result: Result<RoomResponse, ScError> = handle.await?;

        return result;
    }

    async fn spawn_send(
        &self,
        msg: RoomRequest,
    ) -> tokio::task::JoinHandle<Result<RoomResponse, ScError>> {
        let tx = self.tx.clone();
        let (resp_tx, resp_rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            match tx.send((msg, resp_tx)).await {
                Ok(_) => {
                    // tracing::info!("Message sent successfully");
                    return match resp_rx.await {
                        Ok(response) => Ok(response),
                        Err(err) => Err(ScError::OneshotRecieveError(err)),
                    };
                }
                Err(err) => {
                    tracing::info!("Error sending message: {}", err);
                    return Err(ScError::MpscSendError(err));
                }
            }
        });
        return handle;
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RoomEvent> {
        return self.bc_tx.subscribe();
    }
}
