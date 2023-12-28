use serde::Deserialize;
use surrealdb::sql::Thing;
use surrealdb::Notification;

use crate::error::ScError;

pub const ROOM: &str = "room";

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Participant {
    pub name: String,
    pub estimate: String,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Room {
    pub id: Thing,
    pub show: bool,
    pub participants: Vec<Participant>,
}

impl Room {
    pub fn new(id: Thing) -> Self {
        Room {
            id,
            show: false,
            participants: vec![],
        }
    }
}

pub fn get_room(result: surrealdb::Result<Notification<Room>>) -> Result<Room, ScError> {
    match result {
        Ok(notification) => {
            let _action = notification.action;
            let room = notification.data;
            Ok(room)
        }
        Err(error) => Err(error.into()),
    }
}
