use std::borrow::Borrow;

use crate::{
    app_state::{self, AppState},
    lib::{authentication::Authentication, id::ID},
};
use app_state::{event::Event, sessions::Session};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
    pub handle: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    AuthenticationFailed,
    SessionNotFound,
}

pub async fn run<'a>(state: AppState, input: Input) -> Output {
    let store = state.get_store().await;

    if store.sessions.get(&input.session_id).is_none() {
        return Output::SessionNotFound;
    }

    let user_ref = match store.users.get_by_handle(&input.handle) {
        Some(user) => user,
        None => return Output::AuthenticationFailed,
    };
    let user = user_ref.read().await;

    if user.password != input.password {
        return Output::AuthenticationFailed;
    }

    drop(store);

    state.write(Event::SessionLogin {
        session_id: input.session_id,
        user_id: user.id.clone(),
    });

    return Output::Success;
}
