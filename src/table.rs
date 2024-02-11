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
            thead { class: "text-xs text-gray-700 uppercase bg-gray-50",
                tr {
                    th { scope: "col", class: "py-3 px-6", "Name" }
                    th { scope: "col", class: "py-3 px-6 text-center", "Story Points" }
                }
            }
            tbody {
                for (_ , participant) in participants.read().iter() {
                    tr { class: "bg-white border-b",
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
    let estimate_placed = if estimate.is_empty() { "-" } else { "o" };
    if *show {
        cx.render(rsx! {
            div { span { "{estimate}" } }
        })
    } else {
        cx.render(rsx! {
            div { span { "{estimate_placed}" } }
        })
    }
}
