use crate::{error::ScError, estimate::Estimate, room::Participant};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum RoomRequest {
    Join(Participant),
    Leave(Uuid),
    Remove(Uuid),
    SendEstimate(Uuid, Estimate),
    ChangeVisibility,
    DeleteEstimates,
    Heartbeat(Uuid),
    NameChange(Uuid, Arc<str>),
}

#[derive(Clone, Debug)]
pub enum RoomResponse {
    ListParticipants(HashMap<Uuid, Participant>),
    EstimateRecieved,
}

#[derive(Clone, Debug)]
pub enum RoomEvent {
    Joined(Participant),
    ParticipantUpdate(Participant),
    ChangedVisibility(EstimateVisibility),
    EstimatesDeleted,
    Left(Uuid),
    RoomRequestedHeartbeat,
}

#[derive(PartialEq, Clone, Debug)]
pub enum EstimateVisibility {
    Visible,
    Hidden,
}

impl EstimateVisibility {
    pub fn toggle(&self) -> Self {
        match *self {
            EstimateVisibility::Visible => EstimateVisibility::Hidden,
            EstimateVisibility::Hidden => EstimateVisibility::Visible,
        }
    }

    pub fn is_visible(&self) -> bool {
        if *self == EstimateVisibility::Visible {
            return true;
        } else {
            return false;
        }
    }
}

pub type RoomMessage = (RoomRequest, oneshot::Sender<RoomResponse>);

#[derive(Clone, Debug)]
pub struct RoomChannel {
    pub tx: mpsc::Sender<RoomMessage>,
    pub broadcast: broadcast::Sender<RoomEvent>,
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
        return self.broadcast.subscribe();
    }
}
