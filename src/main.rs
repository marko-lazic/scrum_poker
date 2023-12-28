#![allow(non_snake_case)]

use std::sync::Arc;

use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;
use fermi::{use_init_atom_root, Atom};
use tower_http::services::ServeDir;

use crate::{
    app::App,
    pool::{Manager, Pool},
};

mod app;
mod card;
mod db;
mod error;
mod pool;
mod room;
mod table;

pub static RESULTS: Atom<String> = Atom(|_| "".to_string());

#[derive(Clone)]
pub struct AppProps {
    pool: Arc<Pool>,
}

#[tokio::main]
async fn main() {
    let mgr = Manager {};
    let pool = Pool::builder(mgr).max_size(50).build().unwrap();

    let app_props = AppProps {
        pool: Arc::new(pool),
    };
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
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
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view
                        .launch_with_props::<AppProps>(
                            dioxus_liveview::axum_socket(socket),
                            app,
                            app_props,
                        )
                        .await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(cx: Scope<AppProps>) -> Element {
    use_init_atom_root(cx);
    use_shared_state_provider(cx, || cx.props.pool.clone());
    cx.render(rsx! { App {} })
}
