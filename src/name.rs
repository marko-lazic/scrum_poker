use crate::{app::use_app_props, channel::RoomRequest, validate};
use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub fn Name(username: Signal<String>) -> Element {
    let app_props = use_app_props();

    let mut pen_visibility = use_signal(|| false);
    let pen_hidden = if pen_visibility() { "" } else { "hidden" };

    use_hook({
        move || {
            let create_eval = eval(
                r#"
                let usernameFromServer = await dioxus.recv();
                document.getElementById('nameInput').value = usernameFromServer;
                "#,
            );
            create_eval.send(username().into()).unwrap();
        }
    });

    let oninput_send = r#"
    dioxus.send(window.username);
    "#;

    let oninput_recv = r#"
    let validatedUsername = await dioxus.recv();
    document.getElementById('nameInput').value = validatedUsername;
    window.username = validatedUsername;
    "#;
    rsx! {
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
                        _ = eval(r#"document.getElementById("nameInput").blur();"#);
                        pen_visibility.set(false);
                    }
                },
                onfocusout: move |_| {
                    let mut name_eval = eval(oninput_send);
                    async move {
                        let recieved_name = name_eval.recv().await.unwrap();
                        let validated_name = validate::username(
                            &recieved_name.as_str().unwrap().to_string(),
                        );
                        username.set(validated_name.clone());
                        let recv_name_eval = eval(oninput_recv);
                        recv_name_eval.send(validated_name.clone().into()).unwrap();
                        app_props().session.set("username", validated_name.clone());
                        _ = app_props()
                            .channel
                            .send(
                                RoomRequest::NameChange(
                                    app_props().session_id,
                                    Arc::from(validated_name.to_owned()),
                                ),
                            )
                            .await;
                    }
                },
                "oninput": "window.username = document.getElementById('nameInput').value;",
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
    }
}
