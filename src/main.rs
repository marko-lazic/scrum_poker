#![allow(non_snake_case)]

use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;
use tower_http::services::ServeDir;

use crate::{app::App, card::Card};

mod app;
mod card;
mod demo;

#[tokio::main]
async fn main() {
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
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(cx: Scope) -> Element {
    cx.render(
        rsx! {
            App {}
            div { class: "h-10" }
            div { class: "p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4",
                div { class: "shrink-0", img {
                    class: "h-12 w-12",
                    src: "/public/rustore-svgrepo-com.svg",
                    alt: "ChitChat Logo"
                } }
                div {
                    "ChitChat"
                    p { class: "text-slate-500", "You have a new message!" }
                }
            }
            div { class: "h-10" }
            div { class: "py-8 px-8 max-w-sm mx-auto bg-white rounded-xl shadow-lg space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6",
                img {
                    class: "block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0",
                    src: "/public/logo_trans.png",
                    alt: "Scrum poker logo"
                }
                div { class: "text-center space-y-2 sm:text-left",
                    div { class: "space-y-0.5",
                        p { class: "text-lg text-black font-semibold", "Scrum Poker Online" }
                        p { class: "text-slate-500 font-medium", "For Software Engineers" }
                    }
                    button { class: "px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border border-purple-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2",
                        "Play Now"
                    }
                }
            }
            div { class: "h-10" }
        })
}
