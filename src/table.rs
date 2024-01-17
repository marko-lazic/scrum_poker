use std::{collections::HashSet, sync::Arc};

use dioxus::prelude::*;

use crate::{
    channel::{use_room_channel, RoomEvent, RoomRequest, RoomResponse},
    room::Participant,
    session::use_session_id,
};

#[component]
pub fn Table(cx: Scope) -> Element {
    let participants = use_ref(cx, || HashSet::<Participant>::new());
    let channel = use_room_channel(cx);
    let session_id = use_session_id(cx);

    use_future(cx, (), move |_| {
        let channel = channel.clone();
        let participants = participants.clone();

        let name = names::Generator::default().next().unwrap_or_default();
        let participant = Participant::new(session_id.read().clone(), Arc::new(name.clone()));
        let add_participant = RoomRequest::AddParticipant(participant);

        async move {
            let mut rx = channel.read().subscribe();
            let result = channel.read().send(add_participant).await;

            match result {
                RoomResponse::ListParticipants(participants_list) => {
                    *participants.write() = participants_list;
                }
            }

            loop {
                let result = rx.recv().await;
                match result {
                    Ok(msg) => match msg {
                        RoomEvent::ParticipantJoined(p) => {
                            participants.write().insert(p.clone());
                        }
                    },
                    Err(err) => println!("Table component recieved err {:?}", err),
                }
            }
        }
    });
    cx.render(rsx! {
        table { class: "w-full text-sm text-left text-gray-500 dark:text-gray-400",
            thead { class: "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400",
                tr {
                    th { scope: "col", class: "py-3 px-6", "Name" }
                    th { scope: "col", class: "py-3 px-6 text-center", "Story Points" }
                }
            }
            tbody {
                for participant in participants.read().iter() {
                    tr { class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
                        td { class: "py-3 px-6", "{participant.name}" }
                        td { class: "py-3 px-6 text-center", "{participant.estimate}" }
                    }
                }
            }
        }
    })
}
