#![allow(non_snake_case)]

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::{Html, Redirect, Response},
    routing::get,
    Router,
};
use axum_session::{
    SessionConfig, SessionLayer, SessionStore, SessionSurrealPool, SessionSurrealSession,
};
use nanoid::nanoid;
use std::sync::Arc;

use fermi::Atom;
use surrealdb::engine::remote::ws::Client;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{
    app::App,
    pool::{Manager, Pool},
};

mod app;
mod card;
mod error;
mod pool;
mod room;
mod table;

pub static RESULTS: Atom<String> = Atom(|_| "".to_string());

#[derive(Clone)]
pub struct AppState {
    pub addr: std::net::SocketAddr,
    pub pool: Arc<Pool>,
    pub view: dioxus_liveview::LiveViewPool,
}

impl AppState {
    fn new() -> AppState {
        let mgr = Manager {};
        let pool = Pool::builder(mgr).max_size(50).build().unwrap();
        AppState {
            addr: ([127, 0, 0, 1], 3030).into(),
            pool: Arc::new(pool),
            view: dioxus_liveview::LiveViewPool::new(),
        }
    }
}

#[derive(Clone)]
pub struct AppProps {
    pool: Arc<Pool>,
    session_id: Uuid,
    room_id: String,
}

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let addr = app_state.addr;
    // Axum session
    let session_config = SessionConfig::default();
    // create SessionStore and initiate the database tables
    let surr_db = app_state.pool.clone().get().await.unwrap().clone();

    let session_store = SessionStore::new(Some(SessionSurrealPool::new(surr_db)), session_config)
        .await
        .unwrap();

    let routes = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(root))
        .route("/:room_id", get(room_handler))
        .route("/ws/:room_id", get(ws_handler))
        .with_state(app_state)
        .layer(SessionLayer::new(session_store));

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Redirect {
    let room_id = nanoid!(10);
    println!("Creating room id {}", room_id);
    Redirect::to(format!("/{room_id}").as_str())
}

async fn room_handler(State(state): State<AppState>, Path(room_id): Path<String>) -> Html<String> {
    let addr = state.addr;
    Html(format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Dioxus LiveView with Axum</title>
        <link rel="stylesheet" href="/public/tailwind.css">
        <link rel="stylesheet" href="/public/style.css">
        <script src="https://cdn.tailwindcss.com"></script>
    </head>
    <body> <div id="main"></div> </body>
    {glue}
    </html>
    "#,
        // Create the glue code to connect to the WebSocket on the "/ws" route
        glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws/{room_id}"))
    ))
}

async fn ws_handler(
    Path(room_id): Path<String>,
    ws: WebSocketUpgrade,
    session: SessionSurrealSession<Client>,
    State(state): State<AppState>,
) -> Response {
    let get_session_id = session.get_session_id();
    let session_id = get_session_id.uuid();
    // TODO: Check if room exists. Create new or join existing room.
    // TODO: Give user a handle to a room
    // TODO: Allow user to change estimate for himself
    println!("Joining session id {} room id {}", session_id, room_id);

    ws.on_upgrade(move |socket| websocket(socket, state, session_id, room_id))
}

async fn websocket(stream: WebSocket, state: AppState, session_id: Uuid, room_id: String) {
    let app_props = AppProps {
        pool: state.pool,
        session_id,
        room_id,
    };

    _ = state
        .view
        .launch_with_props::<AppProps>(dioxus_liveview::axum_socket(stream), App, app_props)
        .await;
}
