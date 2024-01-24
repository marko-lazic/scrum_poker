use crate::{
    app::use_app_props,
    channel::{EstimateData, RoomRequest},
};
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: Arc<str>,
}

#[component]
pub fn Card(cx: Scope<CardProps>) -> Element {
    let app_props = use_app_props(cx);

    cx.render(rsx! {
        button {
            class: "select-none p-1 relative w-12 md:w-20 h-14 md:h-28 mx-auto bg-white hover:bg-slate-100 focus:bg-slate-400 rounded-xl shadow-lg text-2xl md:text-3xl text-slate-500 focus:text-slate-50",
            onclick: move |_| {
                tracing::info!("Card clicked {:?}", cx.props.value);
                let app_props = app_props.read().clone();
                let value = cx.props.value.clone();
                async move {
                    let e = EstimateData {
                        session_id: app_props.session_id,
                        value: value,
                    };
                    let result = app_props.channel.send(RoomRequest::Estimate(e)).await;
                    match result {
                        Ok(response) => {
                            tracing::info!("Card {:?}", response);
                        }
                        Err(err) => {
                            tracing::info!("Card estimate send error {:?}", err);
                        }
                    }
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
