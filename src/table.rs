use crate::{channel::EstimateVisibility, estimate::Estimate, room::Participant};
use dioxus::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;
use uuid::Uuid;

#[component]
pub fn Table(
    participants: Signal<HashMap<Uuid, Participant>>,
    estimate_visibility: Signal<EstimateVisibility>,
) -> Element {
    let p = participants.read();
    let participants: Vec<(&Uuid, &Participant)> = if estimate_visibility().is_visible() {
        let sorted_vec: Vec<_> = p.iter().sorted_by_key(|x| x.1.estimate.clone()).collect();
        sorted_vec
    } else {
        let unsorted_vec: Vec<_> = p.iter().collect();
        unsorted_vec
    };

    rsx! {
        table { class: "w-full text-sm text-left text-gray-500",
            thead { class: "text-base text-gray-700 uppercase bg-gray-100",
                tr {
                    th { scope: "col", class: "py-3 px-6", "Name" }
                    th { scope: "col", class: "py-3 px-6 text-center", "Story Points" }
                }
            }
            tbody { class: "text-lg",
                for (_ , participant) in participants {
                    tr { class: "bg-gray-50  border-b",
                        td { class: "py-3 px-6", "{participant.name}" }
                        td { class: "py-3 px-6 text-center",
                            EstimateResultCard {
                                estimate: participant.estimate.clone(),
                                show: estimate_visibility().is_visible()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EstimateResultCard(estimate: Estimate, show: bool) -> Element {
    let has_estimate = if estimate == Estimate::None {
        false
    } else {
        true
    };
    if show {
        rsx! {
            div { class: "flex items-center justify-center p-1 w-8 h-11 mx-auto bg-white rounded-md shadow-md text-slate-500",
                span { "{estimate}" }
            }
        }
    } else {
        rsx! {
            div { class: "flex items-center justify-center p-1 w-8 h-11 mx-auto bg-white rounded-md shadow-md text-slate-500",
                span { HiddenEstimateResultCard { has_estimate: has_estimate.clone() } }
            }
        }
    }
}

#[component]
fn HiddenEstimateResultCard(has_estimate: bool) -> Element {
    if has_estimate {
        rsx! { img { src: "/public/logo_trans.png" } }
    } else {
        rsx! {"-"}
    }
}
