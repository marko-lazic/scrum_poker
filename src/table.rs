use dioxus::prelude::*;
use fermi::*;

use crate::app::RESULTS;

#[component]
pub fn Table(cx: Scope) -> Element {
    let results = use_read(cx, &RESULTS);

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
                    td { class: "py-3 px-6 text-center", "-" }
                }
            }
        }
    })
}
