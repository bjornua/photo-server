use app_state::sessions;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::{self, RequestState},
    lib::id::ID,
    permission,
};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
}

#[derive(Serialize)]
pub struct Session {
    token: ID,
    auth_user: Option<String>,
}

#[derive(Serialize)]
pub enum Output {
    Success(Vec<Session>),
    AccessDenied,
}

pub async fn run<'a, T>(state: RequestState<T>, input: Input) -> Output {
    let state = state.get_store().await;

    let authentication = match state.sessions.get(&input.session_id) {
        Some(sessions::Session { authentication, .. }) => authentication,
        None => return Output::AccessDenied,
    };

    if !permission::session_list(authentication) {
        return Output::AccessDenied;
    };

    let mut sessions: Vec<Session> = Vec::new();

    for session in state.sessions.list().into_iter() {
        sessions.push(Session {
            token: session.token.clone(),
            auth_user: match &session.authentication {
                crate::lib::authentication::Authentication::NotAuthenticated => None,
                crate::lib::authentication::Authentication::Authenticated { user } => {
                    Some(user.upgrade().unwrap().read().await.name.clone())
                }
            },
        });
    }

    return Output::Success(sessions);
}
