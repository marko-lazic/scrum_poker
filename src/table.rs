use dioxus::prelude::*;
use fermi::*;
use futures::StreamExt;
use surrealdb::engine::any::connect;
use surrealdb::opt::auth::Root;

use crate::app::RESULTS;
use crate::room::{room_results, ROOM};

#[component]
pub fn Table(cx: Scope) -> Element {
    let results = use_read(cx, &RESULTS);
    let borka_results = use_state(cx, || "-".to_string());

    use_future(cx, (), move |_| {
        let mut borka_results = borka_results.clone();
        async move {
            let db = connect("ws://localhost:8000").await.unwrap();
            db.signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();
            db.use_ns("test").use_db("test").await.unwrap();
            println!("Health {:?}", db.health());
            let mut rooms = db.select(ROOM).range("one".."two").live().await.unwrap();

            while let Some(notification) = rooms.next().await {
                borka_results.set(room_results(notification));
            }
        }
    });

    cx.render(rsx! {
        table { class: "w-full text-sm text-left text-gray-500 dark:text-gray-400",
            thead { class: "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400",
                tr {
                    th { scope: "col", class: "py-3 px-6", "Name" }
                    th { scope: "col", class: "py-3 px-6 text-center", "Story Points" }
                }
            }
            tbody {
                tr { class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
                    td { class: "py-3 px-6", "Marko" }
                    td { class: "py-3 px-6 text-center", "{results}" }
                }
                tr { class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
                    td { class: "py-3 px-6", "Borka" }
                    td { class: "py-3 px-6 text-center", "{borka_results}" }
                }
            }
        }
    })
}
