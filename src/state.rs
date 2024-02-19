use crate::{database, room_handler::RoomHandler};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub addr: std::net::SocketAddr,
    pub pool: Arc<database::Pool>,
    pub view: dioxus_liveview::LiveViewPool,
    pub room_handler: RoomHandler,
}

impl AppState {
    pub fn new() -> AppState {
        let mgr = database::Manager {};
        let pool = database::Pool::builder(mgr).max_size(50).build().unwrap();
        AppState {
            addr: ([127, 0, 0, 1], 3030).into(),
            pool: Arc::new(pool),
            view: dioxus_liveview::LiveViewPool::new(),
            room_handler: RoomHandler::new(),
        }
    }
}
