use dioxus::prelude::*;
use fermi::*;
use futures::StreamExt;

use crate::pool::use_pool;
use crate::room::{get_room, Room, ROOM};
use crate::RESULTS;

#[component]
pub fn Table(cx: Scope) -> Element {
    let pool = use_pool(cx);
    let _results = use_read(cx, &RESULTS);
    let room = use_state(cx, || Room::new(("room", "two").into()));
    use_future(cx, (), move |_| {
        let room = room.clone();
        let pool = pool.clone();
        async move {
            let db = pool
                .read()
                .get()
                .await
                .expect("Failed to get connection from pool");
            if let Some(initial_room) = db.select((ROOM, "two")).await.unwrap() {
                room.set(initial_room);
            }

            let mut rooms = db.select((ROOM, "two")).live().await.unwrap();

            while let Some(notification) = rooms.next().await {
                let result = get_room(notification);
                match result {
                    Ok(value) => room.set(value),
                    Err(error) => println!("{}", error),
                }
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
                for participant in room.participants.clone() {
                    tr { class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
                        td { class: "py-3 px-6", "{participant.name}" }
                        td { class: "py-3 px-6 text-center", "{participant.estimate}" }
                    }
                }
            }
        }
    })
}
