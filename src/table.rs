use crate::{channel::EstimateVisibility, room::Participant};
use dioxus::prelude::*;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

#[component]
pub fn Table(
    cx: Scope,
    participants: UseRef<HashMap<Uuid, Participant>>,
    visibility: UseState<EstimateVisibility>,
) -> Element {
    cx.render(rsx! {
        table { class: "w-full text-sm text-left text-gray-500",
            thead { class: "text-base text-gray-700 uppercase bg-gray-100",
                tr {
                    th { scope: "col", class: "py-3 px-6", "Name" }
                    th { scope: "col", class: "py-3 px-6 text-center", "Story Points" }
                }
            }
            tbody { class: "text-lg",
                for (_ , participant) in participants.read().iter() {
                    tr { class: "bg-gray-50  border-b",
                        td { class: "py-3 px-6", "{participant.name}" }
                        td { class: "py-3 px-6 text-center",
                            Estimate { estimate: participant.estimate.clone(), show: visibility.is_visible() }
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn Estimate(cx: Scope, estimate: Arc<str>, show: bool) -> Element {
    let has_estimate = if estimate.is_empty() { false } else { true };
    if *show {
        cx.render(rsx! {
            div { class: "flex items-center justify-center p-1 w-8 h-11 mx-auto bg-white rounded-md shadow-md text-slate-500",
                span { "{estimate}" }
            }
        })
    } else {
        cx.render(rsx! {
            div { class: "flex items-center justify-center p-1 w-8 h-11 mx-auto bg-white rounded-md shadow-md text-slate-500",
                span { HiddenEstimate { has_estimate: has_estimate.clone() } }
            }
        })
    }
}

#[component]
fn HiddenEstimate(cx: Scope, has_estimate: bool) -> Element {
    if *has_estimate {
        cx.render(rsx! { img { src: "/public/logo_trans.png" } })
    } else {
        cx.render(rsx! {"-"})
    }
}
