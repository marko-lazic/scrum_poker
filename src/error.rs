use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};

use crate::channel::RoomMessage;

#[derive(thiserror::Error, Debug)]
pub enum ScError {
    #[error("failed to retrieve from database")]
    DatabaseError(#[from] surrealdb::Error),
    #[error("send error: {0}")]
    MpscSendError(#[from] mpsc::error::SendError<RoomMessage>),
    #[error("oneshot error: {0}")]
    OneshotRecieveError(#[from] oneshot::error::RecvError),
    #[error("tokio task join error: {0}")]
    TokioJoinError(#[from] tokio::task::JoinError),
}
