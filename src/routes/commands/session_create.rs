use serde::{Deserialize, Serialize};

use crate::{
    app_state::{log::Writer, RequestState},
    lib::id::Id,
};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success { session_id: Id },
}

pub async fn run<'a, T: Writer>(state: RequestState<T>, _input: Input) -> Output {
    let session_id = Id::new();

    state
        .write(crate::app_state::event::Event::SessionCreate {
            session_id: session_id.clone(),
        })
        .await;

    return Output::Success { session_id };
}
