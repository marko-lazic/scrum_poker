use serde::Deserialize;
use surrealdb::sql::Thing;
use surrealdb::Notification;
use surrealdb::Result;

pub const ROOM: &str = "room";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Room {
    id: Thing,
    result: String,
}

pub fn room_results(result: Result<Notification<Room>>) -> String {
    match result {
        Ok(notification) => {
            let _ = notification.action;
            let room = notification.data;
            room.result
        }
        Err(error) => error.to_string(),
    }
}
