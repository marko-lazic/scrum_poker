use dioxus::prelude::*;
use fermi::*;

use crate::pool::use_pool;
use crate::room::Room;
use crate::RESULTS;

#[component]
pub fn Table(cx: Scope) -> Element {
    let pool = use_pool(cx);
    let _results = use_read(cx, &RESULTS);
    let room = use_state(cx, || Room::new(("room", "two").into()));
    use_future(cx, (), move |_| {
        let _room = room.clone();
        let pool = pool.clone();
        async move {
            let _db = pool
                .read()
                .get()
                .await
                .expect("Failed to get connection from pool");
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
