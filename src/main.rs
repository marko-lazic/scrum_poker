#![allow(non_snake_case)]

use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;
use tower_http::services::ServeDir;

use crate::card::Card;

mod card;
mod player;

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
    cx.render(rsx! {
        div {
            header { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                        StacksIcon {}
                        span { class: "ml-3 text-xl", "Scrum Poker" }
                    }
                    nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                        a { class: "mr-5 hover:text-white", "First Link" }
                        a { class: "mr-5 hover:text-white", "Second Link" }
                        a { class: "mr-5 hover:text-white", "Third Link" }
                        a { class: "mr-5 hover:text-white", "Fourth Link" }
                    }
                    button { class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                        "Button"
                        RightArrowIcon {}
                    }
                }
            }

            section { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                        div { class: "content",
                            span { width: "3rem", height: "3rem", button { "?" } }
                            span { width: "3rem", height: "3rem", button { "☕️" } }
                            span { width: "3rem", height: "3rem", button { "0" } }
                            span { width: "3rem", height: "3rem", button { "0.5" } }
                            span { width: "3rem", height: "3rem", button { "1" } }
                            span { width: "3rem", height: "3rem", button { "2" } }
                            span { width: "3rem", height: "3rem", button { "3" } }
                            span { width: "3rem", height: "3rem", button { "5" } }
                        }
                        div { class: "content",
                            span { width: "3rem", height: "3rem", button { "8" } }
                            span { width: "3rem", height: "3rem", button { "13" } }
                            span { width: "3rem", height: "3rem", button { "20" } }
                            span { width: "3rem", height: "3rem", button { "40" } }
                            span { width: "3rem", height: "3rem", button { "100" } }
                        }
                    }
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6" }
                }
            }
        }
    })
}

pub fn StacksIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
        }
    ))
}

pub fn RightArrowIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    ))
}
