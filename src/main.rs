#![allow(non_snake_case)]

use crate::{app::App, state::AppState};
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
use channel::RoomChannel;
use nanoid::nanoid;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use tower_http::services::ServeDir;
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;

mod app;
mod card;
mod channel;
mod database;
mod error;
mod room;
mod state;
mod table;
mod username;

#[derive(Clone)]
pub struct AppProps {
    // TODO: Use or remove pool
    _pool: Arc<database::Pool>,
    session_id: Uuid,
    room_id: Arc<str>,
    channel: RoomChannel,
    username: Arc<str>,
}

#[tokio::main]
async fn main() {
    init_tracing();
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

    tracing::info!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Redirect {
    // TODO: nanoid currently can make _ -
    let room_id = nanoid!(10);
    tracing::info!("Creating room id {}", room_id);
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
    Path(room_id): Path<Arc<str>>,
    ws: WebSocketUpgrade,
    session: SessionSurrealSession<Client>,
    State(state): State<AppState>,
) -> Response {
    let session_id = session.get_session_id().uuid();
    let channel = state.spawn_or_find_room(room_id.clone()).await;
    let username = username::get_username(session);

    ws.on_upgrade(move |socket| websocket(socket, state, session_id, room_id, channel, username))
}

async fn websocket(
    stream: WebSocket,
    state: AppState,
    session_id: Uuid,
    room_id: Arc<str>,
    channel: RoomChannel,
    username: Arc<str>,
) {
    let app_props = AppProps {
        _pool: state.pool,
        session_id,
        room_id,
        channel,
        username,
    };

    _ = state
        .view
        .launch_with_props::<AppProps>(dioxus_liveview::axum_socket(stream), App, app_props)
        .await;
}

fn init_tracing() {
    // Start configuring a fmt
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on with_thread_ids (true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
