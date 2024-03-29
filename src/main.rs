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
use room::RoomId;
use std::sync::Arc;
use surrealdb::engine::any::Any;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod actions;
mod app;
mod channel;
mod database;
mod deck;
mod error;
mod estimate;
mod logs;
mod name;
mod room;
mod room_pool;
mod state;
mod table;
mod username;
mod validate;

#[derive(Clone)]
pub struct AppProps {
    // TODO: Use or remove pool
    pub _pool: Arc<database::Pool>,
    pub session: SessionSurrealSession<Any>,
    pub session_id: Uuid,
    pub room_id: RoomId,
    pub channel: RoomChannel,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
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

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Redirect {
    let room_id = nanoid::nanoid!(10, &ALPHABET_AND_NUMBERS);
    tracing::trace!("Create new room id {}", room_id);
    Redirect::to(format!("/{room_id}").as_str())
}

async fn room_handler(State(state): State<AppState>, Path(room_id): Path<RoomId>) -> Response {
    let validated_room_id = validate::room_id(room_id.clone());

    if validated_room_id != room_id.clone() {
        let redirect = Redirect::to(format!("/{validated_room_id}").as_str());
        return redirect.into_response();
    }

    let ws_addr = state.ws_addr;
    let html = Html(format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Scrum Poker</title>
        <meta name="color-scheme" content="light only" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" type="image/x-icon" href="/public/favicon.ico" />
        <link rel="stylesheet" href="/public/tailwind.css" />
        <script src="/public/sp.js"></script>
    </head>
    <body> <div id="main"></div> </body>
    {glue}
    </html>
    "#,
        // Create the glue code to connect to the WebSocket on the "/ws" route
        glue = dioxus_liveview::interpreter_glue(&format!("{ws_addr}/ws/{room_id}"))
    ));

    html.into_response()
}

async fn ws_handler(
    Path(room_id): Path<RoomId>,
    ws: WebSocketUpgrade,
    session: SessionSurrealSession<Any>,
    State(state): State<AppState>,
) -> Response {
    let session_id = session.get_session_id().uuid();
    let Ok(channel) = state.room_pool.spawn(&room_id).await else {
        tracing::error!("Failed to spawn room_id: {}", room_id);
        return root().await.into_response();
    };

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
