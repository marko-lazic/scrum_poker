use std::collections::HashMap;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    app::use_app_props,
    channel::{RoomEvent, RoomRequest, RoomResponse},
    room::Participant,
};

#[component]
pub fn Table(cx: Scope) -> Element {
    let participants = use_ref(cx, || HashMap::<Uuid, Participant>::new());
    let app_props = use_app_props(cx);

    use_on_destroy(cx, {
        let app_props = app_props.read().clone();
        move || {
            let channel = app_props.channel.clone();
            tokio::task::spawn(async move {
                _ = channel.send(RoomRequest::Leave(app_props.session_id)).await;
                tracing::trace!(
                    "Table component removed. Send ParticipantLeft {}",
                    app_props.session_id
                );
            });
        }
    });

    use_future(cx, (), move |_| {
        let app_props = app_props.read().clone();
        let participants = participants.clone();

        let participant = Participant::new(app_props.session_id, app_props.username);
        let add_participant = RoomRequest::Join(participant);

        async move {
            let mut rx = app_props.channel.subscribe();
            let result = app_props.channel.send(add_participant).await;

            match result {
                Ok(response) => match response {
                    RoomResponse::ListParticipants(participants_list) => {
                        *participants.write() = participants_list;
                    }
                    _ => {}
                },
                Err(err) => {
                    tracing::error!(
                        "Failed to get list of participants, room_id {}, error: {:?}",
                        app_props.room_id,
                        err
                    );
                }
            }

            loop {
                let result = rx.recv().await;
                match result {
                    Ok(msg) => match msg {
                        RoomEvent::Joined(p) => {
                            participants.write().insert(p.session_id, p);
                        }
                        RoomEvent::Update(p) => {
                            participants.write().insert(p.session_id, p);
                        }
                        RoomEvent::Left(session_id) => {
                            participants.write().remove(&session_id);
                        }
                    },
                    Err(err) => tracing::info!(
                        "Failed to get room event, room_id: {}, error {:?}",
                        app_props.room_id,
                        err
                    ),
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
                for (_ , participant) in participants.read().iter() {
                    tr { class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
                        td { class: "py-3 px-6", "{participant.name}" }
                        td { class: "py-3 px-6 text-center", "{participant.estimate}" }
                    }
                }
            }
        }
    })
}
