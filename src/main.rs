#![allow(non_snake_case)]

mod player;
mod card;

use std::fmt::Display;
use dioxus::prelude::*;
use crate::card::Card;
use crate::player::Player;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let players = vec![
        Player { name: "cart".to_string(), vote: Card::new(8) },
        Player { name: "Alice".to_string(), vote: Card::none() },
        Player { name: "iyes iða".to_string(), vote: Card::new(5) },
    ];
    cx.render(rsx! {
        h1 {
            "Scrum Poker Online"
        }
        div {
            "Table"
            ul {
                players.iter().map(|player| cx.render(rsx! {
                    li {
                        h3 { "{player.name}" }
                        span {"{player.vote}"}
                    }
                }))
            }
        }
    })
}
