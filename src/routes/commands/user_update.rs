use crate::{
    app_state::{sessions::Session, AppState},
    lib::id::ID,
    permission,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
    pub user_id: ID,
    pub user_handle: String,
    pub user_password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success { id: ID, name: String },
    UserNotFound,
    SessionNotFound,
    AccessDenied,
    NotAuthenticated,
    InvalidSessionID,
}

pub async fn run(state: &AppState, input: Input) -> Output {
    {
        let state = state.read().await;

        let authentication = match state.get_session(&input.session_id) {
            Some(Session { authentication, .. }) => authentication,
            None => return Output::SessionNotFound,
        };

        let target_user = match state.get_user(&input.user_id) {
            Some(user) => user,
            None => return Output::UserNotFound,
        };

        if !permission::user_update(authentication, &*target_user) {
            return Output::AccessDenied;
        }
    };

    state.write().await.get_user_mut(input.session_id);

    return Output::Success {};
}
