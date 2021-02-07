use crate::{
    app_state::{sessions::Session, RequestState},
    lib::id::ID,
    permission,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
    pub user_id: ID,
    pub user_name: String,
    pub user_handle: String,
    pub user_password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    UserNotFound,
    SessionNotFound,
    AccessDenied,
    NotAuthenticated,
    InvalidSessionID,
}

pub async fn run(state: &RequestState, input: Input) -> Output {
    let state = state.get_store().await;

    let authentication = match state.sessions.get(&input.session_id) {
        Some(Session { authentication, .. }) => authentication,
        None => return Output::SessionNotFound,
    };

    let target_user_ref = match state.users.get_by_id(&input.user_id) {
        Some(user) => user,
        None => return Output::UserNotFound,
    };

    if !permission::user_update(authentication, &*target_user_ref.read().await).await {
        return Output::AccessDenied;
    }

    let mut target_user = target_user_ref.write().await;

    target_user.password = input.user_password;
    target_user.name = input.user_name;

    return Output::Success;
}
