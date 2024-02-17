use crate::{error::ScError, room::Participant};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum RoomRequest {
    Join(Participant),
    Leave(Uuid),
    Remove(Uuid),
    Estimate(Uuid, Estimate),
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Estimate {
    None,
    QuestionMark,
    Coffe,
    Zero,
    Half,
    One,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
    Twenty,
    Fourty,
    Hundred,
}

impl std::fmt::Display for Estimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string: Arc<str> = self.clone().into();
        write!(f, "{}", display_string)
    }
}

impl From<Estimate> for Arc<str> {
    fn from(estimate: Estimate) -> Arc<str> {
        match estimate {
            Estimate::None => "".into(),
            Estimate::QuestionMark => "?".into(),
            Estimate::Coffe => "☕️".into(),
            Estimate::Zero => "0".into(),
            Estimate::Half => "0.5".into(),
            Estimate::One => "1".into(),
            Estimate::Two => "2".into(),
            Estimate::Three => "3".into(),
            Estimate::Five => "5".into(),
            Estimate::Eight => "8".into(),
            Estimate::Thirteen => "13".into(),
            Estimate::Twenty => "20".into(),
            Estimate::Fourty => "40".into(),
            Estimate::Hundred => "100".into(),
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
