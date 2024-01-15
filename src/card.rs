use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    channel::{use_room_channel, EstimateData, RoomRequest},
    session::use_session_id,
};

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: Arc<str>,
}

#[component]
pub fn Card(cx: Scope<CardProps>) -> Element {
    let channel = use_room_channel(cx);
    let session_id = use_session_id(cx);

    cx.render(rsx! {
        button {
            class: "select-none p-1 relative w-12 md:w-20 h-14 md:h-28 mx-auto bg-white hover:bg-slate-100 focus:bg-slate-400 rounded-xl shadow-lg text-2xl md:text-3xl text-slate-500 focus:text-slate-50",
            onclick: move |_| {
                println!("Clicked {:?}", cx.props.value);
                let channel = channel.write().clone();
                let session_id = session_id.read().clone();
                let value = cx.props.value.clone();
                async move {
                    let e = EstimateData {
                        session_id,
                        value: value,
                    };
                    channel.send(RoomRequest::Estimate(e)).await;
                }
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
