use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, lib::id::ID};

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

pub async fn run<'a>(state: &AppState, input: Input) -> Output {
    return match state.write().await.logout(&input.session_id) {
        Some(_) => Output::Success,
        None => Output::SessionNotFound,
    };
}
