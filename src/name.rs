use dioxus::prelude::*;
use keyboard_types::Code;

#[component]
pub fn Name(cx: Scope) -> Element {
    let name = use_state(cx, || "Needy Muscle".to_string());
    let eval_provider = use_eval(cx);
    cx.render(rsx! {
        div { class: "flex items-center justify-between",
            input {
                id: "nameInput",
                r#type: "text",
                class: "bg-transparent w-full focus:outline-none mb-4 text-4xl font-extrabold leading-none tracking-tight text-slate-900 placeholder-slate-500 md:text-5xl lg:text-6x selection:bg-yellow-400",
                value: "{name}",
                placeholder: "Enter Your Name...",
                maxlength: "20",
                autocomplete: "off",
                onkeypress: move |event| {
                    if event.code() == Code::Enter {
                        _ = eval_provider(r#"document.getElementById("nameInput").blur();"#).unwrap();
                    }
                },
                onfocusout: move |_| {},
                oninput: move |evt| {
                    name.set(evt.value.clone());
                }
            }
        }
    })
}
