use dioxus::prelude::*;
use std::time::Duration;

pub fn Demo(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    increment_counter(cx, count);
    cx.render(
        rsx! {
            div {
                header { class: "text-gray-400 bg-gray-900 body-font",
                    div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                        a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                            StacksIcon {}
                            span { class: "ml-3 text-xl", "Scrum Poker" }
                        }
                        nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                            a { class: "mr-5 hover:text-white", "First Link" }
                            a { class: "mr-5 hover:text-white", "Second Link" }
                            a { class: "mr-5 hover:text-white", "Third Link" }
                            a { class: "mr-5 hover:text-white", "Fourth Link" }
                        }
                        button { class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                            "Button"
                            RightArrowIcon {}
                        }
                    }
                }

                section { class: "text-gray-400 bg-gray-900 body-font",
                    div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                        div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                            h1 { "Current count {count}" }
                            div { class: "content",
                                span { width: "3rem", height: "3rem", button { "?" } }
                                span { width: "3rem", height: "3rem", button { "☕️" } }
                                span { width: "3rem", height: "3rem", button { "0" } }
                                span { width: "3rem", height: "3rem", button { "0.5" } }
                                span { width: "3rem", height: "3rem", button { "1" } }
                                span { width: "3rem", height: "3rem", button { "2" } }
                                span { width: "3rem", height: "3rem", button { "3" } }
                                span { width: "3rem", height: "3rem", button { "5" } }
                            }
                            div { class: "content",
                                span { width: "3rem", height: "3rem", button { "8" } }
                                span { width: "3rem", height: "3rem", button { "13" } }
                                span { width: "3rem", height: "3rem", button { "20" } }
                                span { width: "3rem", height: "3rem", button { "40" } }
                                span { width: "3rem", height: "3rem", button { "100" } }
                            }
                        }
                        div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6" }
                    }
                }
            }
        })
}

fn increment_counter(cx: &Scoped<'_>, count: &UseState<i32>) {
    use_future(cx, (), move |_| {
        let mut count = count.clone();
        async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                count += 1;
            }
        }
    });
}

pub fn StacksIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
        }
    ))
}

pub fn RightArrowIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    ))
}
