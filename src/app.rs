use crate::actions::{DeleteEstimatesButton, DeleteEstimatesModal, ShowEstimatesButton};
use crate::channel::{EstimateVisibility, RoomBroadcastMessage, RoomRequest, RoomResponse};
use crate::deck::Deck;
use crate::estimate::Estimate;
use crate::name::Name;
use crate::room::Participant;
use crate::table::Table;
use crate::{username, AppProps};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn use_app_props() -> Signal<AppProps> {
    use_context::<Signal<AppProps>>()
}

pub fn App(props: AppProps) -> Element {
    let app_props = use_signal(|| props);
    use_context_provider(|| app_props);

    let mut username = use_signal(|| username::get_username(&app_props().session));
    let mut participants = use_signal(|| HashMap::<Uuid, Participant>::new());
    let mut estimate_visibility = use_signal(|| EstimateVisibility::Hidden);

    use_drop(move || {
        let channel = app_props().channel;
        let session_id = app_props().session_id;
        tokio::task::spawn(async move {
            _ = channel.send(RoomRequest::Leave(session_id)).await;
            tracing::trace!(
                "Table component removed. Send ParticipantLeft {}",
                session_id
            );
        });
    });

    use_future(move || {
        let participant = Participant::new(app_props().session_id, Arc::from(username()));
        let add_participant = RoomRequest::Join(participant);

        async move {
            let mut rx = app_props().channel.subscribe();
            let result = app_props().channel.send(add_participant).await;

            match result {
                Ok(response) => match response {
                    RoomResponse::RoomState(participants_list, room_visibility) => {
                        if let Some(my_participant) = participants_list.get(&app_props().session_id)
                        {
                            let index = i32::from(my_participant.estimate.clone());
                            if index > -1 {
                                let card_select_eval = eval(
                                    r#"
                                    let index = await dioxus.recv();
                                    var cardInputs = document.getElementsByName("card-radio-input");
                                    cardInputs[index].checked = true;
                                "#,
                                );

                                card_select_eval.send(index.into()).unwrap();
                            }
                        }

                        *participants.write() = participants_list;

                        estimate_visibility.set(room_visibility);
                    }
                },
                Err(err) => {
                    tracing::error!(
                        "Failed to get list of participants, room_id {}, error: {:?}",
                        app_props().room_id,
                        err
                    );
                }
            }

            loop {
                let result = rx.recv().await;
                match result {
                    Ok(msg) => match msg {
                        RoomBroadcastMessage::Joined(p) => {
                            participants.write().insert(p.session_id, p);
                        }
                        RoomBroadcastMessage::ParticipantUpdate(p) => {
                            participants.write().insert(p.session_id, p.clone());
                            if p.session_id == app_props().session_id
                                && p.name.as_ref() != username.read().as_str()
                            {
                                username.set(p.name.to_string());
                            }
                        }
                        RoomBroadcastMessage::ChangedVisibility(v) => {
                            estimate_visibility.set(v);
                        }
                        RoomBroadcastMessage::EstimatesDeleted => {
                            for (_, p) in participants.write().iter_mut() {
                                p.estimate = Estimate::None;
                            }
                            estimate_visibility.set(EstimateVisibility::Hidden);
                            let _card_deselect = eval(
                                r#"
                                var cardInputs = document.getElementsByName("card-radio-input");
                                for (var i = 0; i < cardInputs.length; i++) {
                                    cardInputs[i].checked = false;
                                }
                            "#,
                            );
                        }
                        RoomBroadcastMessage::Left(session_id) => {
                            participants.write().remove(&session_id);
                        }
                        RoomBroadcastMessage::RoomRequestedHeartbeat => {
                            _ = app_props()
                                .channel
                                .send(RoomRequest::Heartbeat(app_props().session_id))
                                .await;
                        }
                    },
                    Err(err) => tracing::info!(
                        "Failed to get room event, room_id: {}, error {:?}",
                        app_props().room_id,
                        err
                    ),
                }
            }
        }
    });

    let delete_estimates_modal_visibility = use_signal(|| false);
    rsx! {
        div { class: "relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12",
            img {
                src: "/public/beams.jpg",
                alt: "",
                class: "absolute left-1/2 top-1/2 max-w-none -translate-x-1/2 -translate-y-1/2",
                width: "1308"
            }
            div { class: "absolute inset-0 bg-[url(/public/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]" }

            div { class: "mx-auto max-w-4xl",
                div { class: "relative flex px-10", Name { username } }
                div { class: "sm:mx-auto sm:max-w-4x px-10 sm:py-10",
                    div { class: "divide-y divide-gray-300/50 ", Deck {} }
                }
                div { class: "relative flex px-10 pt-6 pb-0",
                    h1 { class: "text-slate-600 text-lg font-semibold", "Results" }
                }
                div { class: "relative flex flex-row-reverse px-11 py-5 gap-x-8 md:gap-x-28",
                    ShowEstimatesButton { estimate_visibility: estimate_visibility.clone() }
                    DeleteEstimatesButton {
                        estimate_visibility: estimate_visibility.clone(),
                        delete_estimates_modal_visibility: delete_estimates_modal_visibility.clone()
                    }
                }
                div { class: "m:mx-auto sm:max-w-4x px-10 sm:py-10",
                    div { class: "relative flex overflow-x-auto shadow-md rounded-lg",
                        Table { participants: participants.clone(), visibility: estimate_visibility.clone() }
                    }
                }
                DeleteEstimatesModal { show_modal: delete_estimates_modal_visibility.clone() }
            }
        }
    }
}
