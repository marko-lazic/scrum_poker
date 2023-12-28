use dioxus::prelude::*;
use fermi::*;
use futures::StreamExt;

use crate::pool::use_pool;
use crate::room::{room_results, ROOM};
use crate::RESULTS;

#[component]
pub fn Table(cx: Scope) -> Element {
    let pool = use_pool(cx);
    let results = use_read(cx, &RESULTS);
    let borka_results = use_state(cx, || "-".to_string());

    use_future(cx, (), move |_| {
        let borka_results = borka_results.clone();
        let pool = pool.clone();
        async move {
            let db = pool
                .read()
                .get()
                .await
                .expect("Failed to get connection from pool");
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
                tr { class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
                    td { class: "py-3 px-6", "Ilija" }
                    td { class: "py-3 px-6 text-center", "-" }
                }
            }
        }
    })
}
