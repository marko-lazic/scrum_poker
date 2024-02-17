#![allow(non_snake_case)]

use crate::{app::App, state::AppState, validate::ALPHABET_AND_NUMBERS};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use axum_session::{
    SessionConfig, SessionLayer, SessionStore, SessionSurrealPool, SessionSurrealSession,
};
use channel::RoomChannel;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod actions;
mod app;
mod card;
mod channel;
mod database;
mod error;
mod logs;
mod name;
mod room;
mod state;
mod table;
mod username;
mod validate;

#[derive(Clone)]
pub struct AppProps {
    // TODO: Use or remove pool
    _pool: Arc<database::Pool>,
    session: SessionSurrealSession<Client>,
    session_id: Uuid,
    room_id: Arc<str>,
    channel: RoomChannel,
}

#[tokio::main]
async fn main() {
    logs::init_tracing();
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
    let room_id = nanoid::nanoid!(10, &ALPHABET_AND_NUMBERS);
    tracing::trace!("Create new room id {}", room_id);
    Redirect::to(format!("/{room_id}").as_str())
}

async fn room_handler(State(state): State<AppState>, Path(room_id): Path<Arc<str>>) -> Response {
    let validated_room_id = validate::room_id(room_id.clone());

    if validated_room_id != room_id.clone() {
        let redirect = Redirect::to(format!("/{validated_room_id}").as_str());
        return redirect.into_response();
    }

    let addr = state.addr;
    let html = Html(format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Dioxus LiveView with Axum</title>
        <link rel="stylesheet" href="/public/tailwind.css">
        <link rel="stylesheet" href="/public/style.css">
        <script src="https://cdn.tailwindcss.com"></script>
        <script src="https://cdn.jsdelivr.net/npm/alpinejs@2.8.2"></script>
        <script src="/public/sp.js"></script>
    </head>
    <body> <div id="main"></div> </body>
    {glue}
    </html>
    "#,
        // Create the glue code to connect to the WebSocket on the "/ws" route
        glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws/{room_id}"))
    ));

    return html.into_response();
}

async fn ws_handler(
    Path(room_id): Path<Arc<str>>,
    ws: WebSocketUpgrade,
    session: SessionSurrealSession<Client>,
    State(state): State<AppState>,
) -> Response {
    let session_id = session.get_session_id().uuid();
    let channel = state.spawn_or_find_room(room_id.clone()).await;

    let app_props = AppProps {
        session: session.clone(),
        _pool: state.pool.clone(),
        session_id,
        room_id,
        channel,
    };

    ws.on_upgrade(move |socket| websocket(socket, state, app_props))
}

async fn websocket(stream: WebSocket, state: AppState, app_props: AppProps) {
    _ = state
        .view
        .launch_with_props::<AppProps>(dioxus_liveview::axum_socket(stream), App, app_props)
        .await;
}
