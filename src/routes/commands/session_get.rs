use app_state::sessions;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::{self, log::Writer, RequestState},
    lib::{authentication::Authentication, id::Id},
};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: Id,
}

#[derive(Serialize)]
pub struct Session {
    token: Id,
    auth_user: Option<Id>,
    last_ping: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success(Session),
    AccessDenied,
}

pub async fn run<'a, T: Writer>(state: RequestState<T>, input: Input) -> Output {
    let store = state.get_store().await;

    match store.sessions.get(&input.session_id) {
        Some(sessions::Session {
            authentication: Authentication::Authenticated { user },
            token: _,
            last_ping: last_seen,
        }) => Output::Success(Session {
            token: input.session_id,
            auth_user: Some(user.upgrade().unwrap().read().await.id.clone()),
            last_ping: *last_seen,
        }),
        Some(sessions::Session {
            authentication: Authentication::NotAuthenticated,
            token: _,
            last_ping: last_seen,
        }) => Output::Success(Session {
            token: input.session_id,
            auth_user: None,
            last_ping: *last_seen,
        }),
        None => return Output::AccessDenied,
    }
}
