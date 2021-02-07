use serde::{Deserialize, Serialize};

use crate::{app_state::RequestState, lib::id::ID};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success { session_id: ID },
}

pub async fn run<'a>(state: RequestState, _input: Input) -> Output {
    let session_id = ID::new();

    state
        .write(crate::app_state::event::Event::SessionCreate {
            session_id: session_id.clone(),
        })
        .await;

    return Output::Success { session_id };
}
