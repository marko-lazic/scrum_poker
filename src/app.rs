use crate::actions::{DeleteEstimatesButton, DeleteEstimatesModal, ShowEstimatesButton};
use crate::card::Card;
use crate::channel::{Estimate, EstimateVisibility, RoomEvent, RoomRequest, RoomResponse};
use crate::name::Name;
use crate::room::Participant;
use crate::table::Table;
use crate::{username, AppProps};
use dioxus::prelude::*;
use fermi::use_init_atom_root;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn use_app_props(cx: &ScopeState) -> &UseSharedState<AppProps> {
    use_shared_state::<AppProps>(cx).expect("App props not provided")
}

#[component]
pub fn App(cx: Scope<AppProps>) -> Element {
    use_init_atom_root(cx);
    use_shared_state_provider(cx, || cx.props.clone());

    let app_props = cx.props.clone();
    let username = username::get_username(&app_props.session);

    let username = use_state(cx, || username);

    let participants = use_ref(cx, || HashMap::<Uuid, Participant>::new());
    let estimate_visibility = use_state(cx, || EstimateVisibility::Hidden);

    use_on_destroy(cx, {
        let app_props = cx.props.clone();
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
        let app_props = cx.props.clone();
        let participants = participants.clone();
        let estimate_visibility = estimate_visibility.clone();
        let username = username.clone();

        let participant = Participant::new(app_props.session_id, Arc::from(username.get().clone()));
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
                        RoomEvent::ParticipantUpdate(p) => {
                            participants.write().insert(p.session_id, p.clone());
                            if p.session_id == app_props.session_id
                                && p.name.as_ref() != username.get().as_str()
                            {
                                username.set(p.name.to_string());
                            }
                        }
                        RoomEvent::ChangedVisibility(v) => {
                            estimate_visibility.set(v);
                        }
                        RoomEvent::EstimatesDeleted => {
                            for (_, p) in participants.write().iter_mut() {
                                p.estimate = Estimate::None;
                            }
                            estimate_visibility.set(EstimateVisibility::Hidden);
                        }
                        RoomEvent::Left(session_id) => {
                            participants.write().remove(&session_id);
                        }
                        RoomEvent::RoomRequestedHeartbeat => {
                            _ = app_props
                                .channel
                                .send(RoomRequest::Heartbeat(app_props.session_id))
                                .await;
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

    let debug = use_state(cx, || false);
    if *debug.get() == true {
        cx.render(rsx! {
            div {
                h1 { "{cx.props.session_id}" }
                h1 { "{cx.props.room_id}" }
            }
        });
    }

    let delete_estimates_modal_visibility = use_state(cx, || false);

    cx.render(rsx! {
        div { class: "relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12",
            img {
                src: "/public/beams.jpg",
                alt: "",
                class: "absolute left-1/2 top-1/2 max-w-none -translate-x-1/2 -translate-y-1/2",
                width: "1308"
            }
            div { class: "absolute inset-0 bg-[url(/public/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]" }

            div { class: "mx-auto max-w-4xl",
                div { class: "relative flex px-10", Name { username: username.clone() } }
                div { class: "sm:mx-auto sm:max-w-4x px-10 sm:py-10",
                    div { class: "divide-y divide-gray-300/50 ",
                        div { class: "flex flex-wrap gap-4",
                            Card { value: Estimate::QuestionMark }
                            Card { value: Estimate::Coffe }
                            Card { value: Estimate::Zero }
                            Card { value: Estimate::Half }
                            Card { value: Estimate::One }
                            Card { value: Estimate::Two }
                            Card { value: Estimate::Three }
                            Card { value: Estimate::Five }
                            Card { value: Estimate::Eight }
                            Card { value: Estimate::Thirteen }
                            Card { value: Estimate::Twenty }
                            Card { value: Estimate::Fourty }
                            Card { value: Estimate::Hundred }
                        }
                    }
                }
                div { class: "relative flex px-10 py-2", h1 { class: "text-slate-600 font-semibold", "Results" } }
                div { class: "relative flex px-11 py-5 sm:mx-auto sm:max-w-4x justify-between",
                    div { span { "" } }
                    div { span { "" } }
                    div { span { "" } }
                    DeleteEstimatesButton {
                        estimate_visibility: estimate_visibility.clone(),
                        delete_estimates_modal_visibility: delete_estimates_modal_visibility.clone()
                    }
                    ShowEstimatesButton { estimate_visibility: estimate_visibility.clone() }
                }
                div { class: "m:mx-auto sm:max-w-4x px-10 sm:py-10",
                    div { class: "relative flex overflow-x-auto shadow-md rounded-lg",
                        Table { participants: participants.clone(), visibility: estimate_visibility.clone() }
                    }
                }
                DeleteEstimatesModal { show_modal: delete_estimates_modal_visibility.clone() }
            }
        }
    })
}
