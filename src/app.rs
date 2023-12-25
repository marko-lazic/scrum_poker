use dioxus::prelude::*;

use crate::card::Card;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12",
            img {
                src: "/public/beams.jpg",
                alt: "",
                class: "absolute left-1/2 top-1/2 max-w-none -translate-x-1/2 -translate-y-1/2",
                width: "1308"
            }
            div { class: "absolute inset-0 bg-[url(/public/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]" }
            div { class: "sm:mx-auto sm:max-w-4xl sm:rounded-lg sm:px-10",
                div { class: "divide-y divide-gray-300/50",
                    div { class: "flex flex-wrap gap-4",
                        Card { value: "?".into() }
                        Card { value: "☕️".into() }
                        Card { value: "0".into() }
                        Card { value: "0.5".into() }
                        Card { value: "1".into() }
                        Card { value: "2".into() }
                        Card { value: "3".into() }
                        Card { value: "5".into() }
                        Card { value: "8".into() }
                        Card { value: "13".into() }
                        Card { value: "20".into() }
                        Card { value: "40".into() }
                        Card { value: "100".into() }
                    }
                }
            }
        }
    })
}
