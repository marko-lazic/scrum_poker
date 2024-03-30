use crate::{app::use_app_props, channel::RoomRequest, estimate::Estimate};
use dioxus::prelude::*;

#[component]
pub fn Deck() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap justify-around gap-4",
            oninput: move |evt| {
                println!("{evt:?}");
            },
            Card { estimate: Estimate::QuestionMark }
            Card { estimate: Estimate::Coffe }
            Card { estimate: Estimate::Zero }
            Card { estimate: Estimate::Half }
            Card { estimate: Estimate::One }
            Card { estimate: Estimate::Two }
            Card { estimate: Estimate::Three }
            Card { estimate: Estimate::Five }
            Card { estimate: Estimate::Eight }
            Card { estimate: Estimate::Thirteen }
            Card { estimate: Estimate::Twenty }
            Card { estimate: Estimate::Fourty }
            Card { estimate: Estimate::Hundred }
        }
    }
}

#[component]
pub fn Card(estimate: Estimate) -> Element {
    let app_props = use_app_props();
    let estimate_id = format!("{}-card-btn", estimate);
    rsx! {
        div {
            input {
                r#type: "radio",
                id: "{estimate_id}",
                name: "card-radio-input",
                class: "hidden peer",
                value: "{estimate}",
                onclick: move |_| {
                    tracing::trace!("Card clicked {:?}", estimate);
                    let estimate_value = estimate.clone();
                    async move {
                        _ = app_props()
                            .channel
                            .send(RoomRequest::SendEstimate(app_props().session_id, estimate_value))
                            .await;
                    }
                }
            }
            label {
                r#for: "{estimate_id}",
                class: "select-none inline-flex relative justify-between bg-white rounded-xl shadow-lg text-2xl md:text-3xl text-slate-500 cursor-pointer peer-checked:border-blue-600 peer-checked:text-slate-50 focus:text-slate-50 peer-checked:bg-slate-400 hover:bg-slate-100 hover:text-slate-500",
                span { class: "p-1 w-12 md:w-20 h-14 md:h-28 mx-auto",
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
                        div { class: "m-auto", "{estimate}" }
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
        }
    }
}
