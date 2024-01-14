use dioxus::{
    core::ScopeState,
    hooks::{use_shared_state, UseSharedState},
};
use uuid::Uuid;

pub fn use_session_id(cx: &ScopeState) -> &UseSharedState<Uuid> {
    use_shared_state::<Uuid>(cx).expect("Session id not provided")
}
