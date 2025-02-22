use crate::{channel::RoomMessage, room_pool::RoomPoolMessage};
use std::fmt::Debug;
use tokio::sync::{mpsc, oneshot};

#[derive(thiserror::Error, Debug)]
pub enum ScError {
    #[error("failed to retrieve from database")]
    DatabaseError(#[from] surrealdb::Error),
    #[error("RoomMessage send error: {0}")]
    RoomMessageSendError(#[from] mpsc::error::SendError<RoomMessage>),
    #[error("RoomPoolMessage send error: {0}")]
    RoomPoolMessageSendError(#[from] mpsc::error::SendError<RoomPoolMessage>),
    #[error("oneshot error: {0}")]
    OneshotRecieveError(#[from] oneshot::error::RecvError),
    #[error("tokio task join error: {0}")]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("Unexpected response from server")]
    UnexpectedResponse,
}
