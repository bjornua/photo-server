use app_state::sessions;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::{self, AppState},
    lib::{authentication::Authentication, id::ID},
};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
}

#[derive(Serialize)]
pub struct Session {
    token: ID,
    auth_user: Option<ID>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success(Session),
    AccessDenied,
}

pub async fn run<'a>(state: AppState, input: Input) -> Output {
    let store = state.get_store().await;

    match store.sessions.get(&input.session_id) {
        Some(sessions::Session {
            authentication: Authentication::Authenticated { user },
            token: _,
            last_seen: _,
        }) => Output::Success(Session {
            token: input.session_id,
            auth_user: Some(user.upgrade().unwrap().read().await.id.clone()),
        }),
        Some(sessions::Session {
            authentication: Authentication::NotAuthenticated,
            token: _,
            last_seen: _,
        }) => Output::Success(Session {
            token: input.session_id,
            auth_user: None,
        }),
        None => return Output::AccessDenied,
    }
}
