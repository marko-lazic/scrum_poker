use crate::{
    app::use_app_props,
    channel::{Estimate, RoomRequest},
};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: Estimate,
}

#[component]
pub fn Card(cx: Scope<CardProps>) -> Element {
    let app_props = use_app_props(cx);

    cx.render(rsx! {
        button {
            class: "select-none p-1 relative w-12 md:w-20 h-14 md:h-28 mx-auto bg-white hover:bg-slate-100 focus:bg-slate-400 rounded-xl shadow-lg text-2xl md:text-3xl text-slate-500 focus:text-slate-50",
            onclick: move |_| {
                tracing::trace!("Card clicked {:?}", cx.props.value);
                let app_props = app_props.read().clone();
                let estimate_point = cx.props.value.clone();
                async move {
                    let result = app_props
                        .channel
                        .send(RoomRequest::Estimate(app_props.session_id, estimate_point))
                        .await;
                    match result {
                        Ok(response) => {
                            tracing::trace!("Server received {:?}", response);
                        }
                        Err(err) => {
                            tracing::error!("Card estimate send error {:?}", err);
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
