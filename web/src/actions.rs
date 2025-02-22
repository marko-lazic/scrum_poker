use dioxus::prelude::*;

use crate::{
    app::use_app_props,
    channel::{EstimateVisibility, RoomRequest},
};

const BLACK_BTN_STLYE: &str = "text-white bg-slate-600 hover:bg-slate-500 focus:ring-slate-600";
const WHITE_BTN_STYLE: &str = "text-slate-600 bg-slate-50 hover:bg-slate-100 focus:ring-slate-600";

#[component]
pub fn DeleteEstimatesButton(
    estimate_visibility: Signal<EstimateVisibility>,
    show_delete_modal: Signal<bool>,
) -> Element {
    rsx! {
        button {
            class: "inline-flex items-center justify-center w-auto px-8 py-4 text-base font-bold leading-6 border border-transparent rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2",
            class: if estimate_visibility().is_visible() { "{BLACK_BTN_STLYE}" } else { "{WHITE_BTN_STYLE}" },
            onclick: move |_| {
                show_delete_modal.set(true);
            },
            "Delete Estimates"
        }
    }
}

#[component]
pub fn ShowEstimatesButton(estimate_visibility: Signal<EstimateVisibility>) -> Element {
    let app_props = use_app_props();

    rsx! {
        button {
            class: "inline-flex items-center justify-center w-auto px-8 py-4 text-base font-bold leading-6 border border-transparent rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2",
            class: if estimate_visibility().is_visible() { "{WHITE_BTN_STYLE}" } else { "{BLACK_BTN_STLYE}" },
            onclick: move |_| {
                async move {
                    _ = app_props().channel.send(RoomRequest::ChangeVisibility).await;
                }
            },
            if estimate_visibility().is_visible() {
                "Hide"
            } else {
                "Show"
            }
        }
    }
}

#[component]
pub fn DeleteEstimatesModal(show_modal: Signal<bool>) -> Element {
    let app_props = use_app_props();

    _ = document::eval(
        r#"
                    var btn = document.getElementById("deleteButton");
                    if (btn != null) {
                        btn.focus();
                    }
                    "#,
    );
    if show_modal() {
        rsx! {
            // Background overlay
            div {
                class: "fixed inset-0 transition-opacity",
                aria_hidden: true,
                onclick: move |_| {
                    show_modal.set(false);
                },
                div { class: "absolute inset-0 bg-gray-400 opacity-75" }
            }
            // Modal
            div { class: "fixed z-10 inset-0 overflow-y-auto",
                div { class: "flex items-center justify-center h-screen pt-4 px-4 pb-20 text-center sm:p-0",
                    // Modal panel
                    div {
                        class: "w-full inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                        role: "dialog",
                        aria_modal: "true",
                        aria_labelledby: "modal-headline",
                        div { class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                            // Modal content
                            div { class: "sm:flex sm:items-start",
                                div { class: "mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10",
                                    img {
                                        class: "h-6 w-6 text-red-600",
                                        src: "/assets/exclamation.svg",
                                        alt: "Exclamation mark",
                                    }
                                }
                                div { class: "mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left",
                                    h3 {
                                        class: "text-lg leading-6 font-medium text-gray-900",
                                        id: "modal-headline",
                                        "Delete Estimates"
                                    }
                                    div { class: "mt-2",
                                        p { class: "text-sm text-gray-500",
                                            "Are you sure you want to delete "
                                            span { class: "font-bold", "all estiamtes" }
                                            "? This action cannot be undone."
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row justify-end",
                            button {
                                class: "w-full inline-flex items-center justify-center rounded-full border border-slate-300 px-8 py-4 bg-white text-base font-medium text-slate-600 hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-slate-600 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm",
                                onclick: move |_| {
                                    show_modal.set(false);
                                },
                                "Cancel"
                            }

                            button {
                                id: "deleteButton",
                                class: "mt-3 w-full inline-flex items-center justify-center rounded-full border border-transparent px-8 py-4 bg-red-500 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm",
                                onclick: move |_| {
                                    async move {
                                        _ = app_props().channel.send(RoomRequest::DeleteEstimates).await;
                                        show_modal.set(false);
                                    }
                                },
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    } else {
        return rsx! {};
    }
}
