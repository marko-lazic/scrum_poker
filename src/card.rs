use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: &'static str,
}

pub fn Card(cx: Scope<CardProps>) -> Element {
    cx.render(rsx! {
        button { class: "p-1 relative w-20 h-28 mx-auto bg-white hover:bg-slate-100 focus:bg-stone-300 rounded-xl shadow-lg",
            span {
                div { class: "w-full h-full flex flex-col justify-between",
                    div { class: "flex justify-between",
                        img {
                            class: "block mx-auto h-3 sm:mx-0 sm:shrink-0",
                            src: "/public/logo_trans.png"
                        }
                        img {
                            class: "block mx-auto h-3 sm:mx-0 sm:shrink-0",
                            src: "/public/logo_trans.png"
                        }
                    }
                    div { class: "flex justify-center text-3xl text-slate-500", "{cx.props.value}" }
                    div { class: "flex justify-between",
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
