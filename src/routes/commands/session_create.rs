use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, lib::id::ID};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success { session_id: ID },
}

pub async fn run<'a>(state: &AppState, _input: Input) -> Output {
    return Output::Success {
        session_id: state.write().await.new_session().token.clone(),
    };
}