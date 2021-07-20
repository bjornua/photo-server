use crate::app_state;
use crate::app_state::AppRequest;
use crate::lib::id::Id;
use crate::permission;
use app_state::store::sessions;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct Input {
    pub session_id: Id,
}

#[derive(Serialize)]
pub struct Session {
    token: Id,
    auth_user: Option<String>,
}

#[derive(Serialize)]
pub enum Output {
    Success(Vec<Session>),
    AccessDenied,
}

pub async fn run(state: AppRequest, input: Input) -> Output {
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

    Output::Success(sessions)
}
