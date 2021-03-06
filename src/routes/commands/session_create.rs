use crate::app_state::AppRequest;
use crate::lib::id::Id;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success { session_id: Id },
}

pub async fn run(state: AppRequest, _input: Input) -> Output {
    let session_id = Id::new();

    state
        .write(crate::app_state::event::Event::SessionCreate {
            session_id: session_id.clone(),
        })
        .await;

    Output::Success { session_id }
}
