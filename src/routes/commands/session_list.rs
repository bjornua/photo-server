use app_state::sessions;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::{self, AppState},
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

pub async fn run<'a>(state: &AppState, input: Input) -> Output {
    let state = state.read().await;

    let authentication = match state.get_session(&input.session_id) {
        Some(sessions::Session { authentication, .. }) => authentication,
        None => return Output::AccessDenied,
    };

    if !permission::list_sessions(authentication) {
        return Output::AccessDenied;
    };

    let sessions: Vec<Session> = state
        .list_sessions()
        .into_iter()
        .map(|session| Session {
            token: session.token.clone(),
            auth_user: match &session.authentication {
                crate::lib::authentication::Authentication::NotAuthenticated => None,
                crate::lib::authentication::Authentication::Authenticated { user } => {
                    Some(user.upgrade().unwrap().name.clone())
                }
            },
        })
        .collect();

    return Output::Success(sessions);
}
