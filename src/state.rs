use crate::{
    database,
    room_pool::{RoomPool, RoomPoolChannel},
};
use std::{env, io, net::ToSocketAddrs, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub addr: std::net::SocketAddr,
    pub ws_addr: Arc<str>,
    pub pool: Arc<database::Pool>,
    pub view: dioxus_liveview::LiveViewPool,
    pub room_pool: RoomPoolChannel,
}

impl AppState {
    pub fn new() -> AppState {
        let mgr = database::Manager {};
        let pool = database::Pool::builder(mgr).max_size(50).build().unwrap();
        let hostname = env::var("HOST_ADDRESS").unwrap_or("127.0.0.1:3030".into());
        let addr = Self::resolve_host(hostname.as_str()).expect("AppState failure");
        AppState {
            addr,
            ws_addr: Arc::from(env::var("WS_ADDRESS").unwrap_or("ws://127.0.0.1:3030".into())),
            pool: Arc::new(pool),
            view: dioxus_liveview::LiveViewPool::new(),
            room_pool: RoomPool::spawn(),
        }
    }

    fn resolve_host(hostname_port: &str) -> std::io::Result<std::net::SocketAddr> {
        let socketaddr = hostname_port.to_socket_addrs()?.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("Could not find destination {hostname_port}"),
            )
        })?;
        Ok(socketaddr)
    }
}
