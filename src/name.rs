use crate::{app::use_app_props, channel::RoomRequest, username};
use dioxus::prelude::*;
use keyboard_types::Key;
use std::sync::Arc;

#[component]
pub fn Name(cx: Scope, username: UseState<String>) -> Element {
    let app_props = use_app_props(cx);

    let pen_visibility: &UseState<bool> = use_state(cx, || false);
    let pen_hidden = if **pen_visibility { "" } else { "hidden" };
    let blur_eval_provider = use_eval(cx);
    cx.render(rsx! {
        div { class: "flex items-center justify-between w-full h-full",
            input {
                id: "nameInput",
                r#type: "text",
                class: "bg-transparent w-full focus:outline-none mb-4 text-4xl md:text-5xl lg:text-6xl font-extrabold leading-none tracking-tight text-slate-900 placeholder-slate-500  selection:bg-yellow-400",
                value: "{username}",
                placeholder: "Enter Your Name...",
                maxlength: "20",
                autocomplete: "off",
                onkeypress: move |event| {
                    if event.key() == Key::Enter {
                        _ = blur_eval_provider(r#"document.getElementById("nameInput").blur();"#)
                            .unwrap();
                        pen_visibility.set(false);
                    }
                },
                onfocusout: move |_| {
                    let app_props = app_props.read().clone();
                    let username = username.clone();
                    async move {
                        let cleaned_input: String = username
                            .get()
                            .trim()
                            .chars()
                            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                            .collect::<String>()
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .join(" ");
                        let final_name = if cleaned_input.is_empty() {
                            username::random_username()
                        } else {
                            cleaned_input
                        };
                        if &final_name != username.get().as_str() {
                            username.set(final_name.clone());
                        }
                        app_props.session.set("username", final_name.clone());
                        let new_name_str: Arc<str> = Arc::from(final_name.to_owned());
                        _ = app_props
                            .channel
                            .send(RoomRequest::NameChange(app_props.session_id, new_name_str))
                            .await;
                    }
                },
                oninput: move |evt| {
                    let evt_value: String = evt.value.clone();
                    username.set(evt_value);
                },
                onclick: move |_| {
                    pen_visibility.set(true);
                },
                onmouseover: move |_| {
                    pen_visibility.set(true);
                },
                onmouseout: move |_| {
                    pen_visibility.set(false);
                }
            }
            div { class: "pointer-events-none w-0",
                h1 { class: "{pen_hidden} select-none mb-4 text-4xl font-extrabold leading-none tracking-tight text-slate-900 md:text-5xl lg:text-6x scale-x-[-1]",
                    "✎"
                }
            }
        }
    })
}
