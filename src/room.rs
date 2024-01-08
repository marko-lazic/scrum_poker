use serde::Deserialize;
use surrealdb::sql::Thing;

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
