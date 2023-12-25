use std::sync::Arc;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: Arc<str>,
}

pub fn Card(cx: Scope<CardProps>) -> Element {
    cx.render(rsx! {
        button {
            class: "select-none p-1 relative sm:w-10 md:w-20 sm:h-14 md:h-28 mx-auto bg-white hover:bg-slate-100 focus:bg-slate-400 rounded-xl shadow-lg text-2xl md:text-3xl text-slate-500 focus:text-slate-50",
            onclick: move |_| {
                println!("Clicked {:?}", cx.props.value);
            },
            span {
                div { class: "flex flex-col w-full h-full md:items-stretch md:justify-between sm:items-center sm:justify-center",
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
                    "{cx.props.value}"
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
