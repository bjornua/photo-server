use serde::{Deserialize, Serialize};

use crate::{
    app_state::{event::Event, AppState},
    lib::id::ID,
};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    SessionNotFound,
    InvalidSessionId,
}

pub async fn run<'a>(state: AppState, input: Input) -> Output {
    let store = state.get_store().await;

    if store.sessions.get(&input.session_id).is_none() {
        return Output::SessionNotFound;
    }
    drop(store);

    state
        .write(Event::SessionLogout {
            session_id: input.session_id,
        })
        .await;

    return Output::Success;
}
