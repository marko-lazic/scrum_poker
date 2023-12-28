use std::sync::Arc;

use dioxus::prelude::*;
use fermi::*;

use crate::RESULTS;

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: Arc<str>,
}

#[component]
pub fn Card(cx: Scope<CardProps>) -> Element {
    let set_results = use_set(cx, &RESULTS);
    cx.render(rsx! {
        button {
            class: "select-none p-1 relative w-12 md:w-20 h-14 md:h-28 mx-auto bg-white hover:bg-slate-100 focus:bg-slate-400 rounded-xl shadow-lg text-2xl md:text-3xl text-slate-500 focus:text-slate-50",
            onclick: move |_| {
                set_results(cx.props.value.to_string());
                println!("Clicked {:?}", cx.props.value);
            },
            span {
                div { class: "flex flex-col w-full h-full justify-between",
                    div { class: "hidden md:flex justify-between",
                        img {
                            class: "block mx-auto h-3 sm:mx-0 sm:shrink-0",
                            src: "/public/logo_trans.png"
                        }
                        img {
                            class: "block mx-auto h-3 sm:mx-0 sm:shrink-0",
                            src: "/public/logo_trans.png"
                        }
                    }
                    div { class: "m-auto", "{cx.props.value}" }
                    div { class: "hidden md:flex justify-between",
                        img {
                            class: "block mx-auto h-3 sm:mx-0 sm:shrink-0",
                            src: "/public/logo_trans.png"
                        }
                        img {
                            class: "block mx-auto h-3 sm:mx-0 sm:shrink-0",
                            src: "/public/logo_trans.png"
                        }
                    }
                }
            }
        }
    })
}
