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
use tokio::sync::broadcast::error::SendError;

use fermi::Atom;
use surrealdb::engine::remote::ws::Client;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{app::App, state::AppState};

mod app;
mod card;
mod database;
mod error;
mod room;
mod state;
mod table;

pub static RESULTS: Atom<String> = Atom(|_| "".to_string());

#[derive(Clone)]
pub struct AppProps {
    pool: Arc<database::Pool>,
    session_id: Uuid,
    room_id: Arc<String>,
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
    let room_id: Arc<String> = Arc::from(room_id);

    let channel = state.get_or_create_room_channel(room_id.clone()).await;
    // TODO: Give user a handle to a room
    // TODO: Allow user to change estimate for himself
    let result = channel.tx.send("Participant message!".to_string());
    print_result(result);

    ws.on_upgrade(move |socket| websocket(socket, state, session_id, room_id))
}

async fn websocket(stream: WebSocket, state: AppState, session_id: Uuid, room_id: Arc<String>) {
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

fn print_result(result: Result<usize, SendError<String>>) {
    match result {
        Ok(_) => {
            // println!("Message sent successfully");
        }
        Err(err) => {
            println!("Error sending message: {}", err);
        }
    }
}
