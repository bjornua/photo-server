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
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success {
        id: ID,
        name: String,
        handle: String,
    },
    UserNotFound,
    SessionNotFound,
    AccessDenied,
}

pub async fn run<'a>(state: RequestState, input: Input) -> Output {
    let store = state.get_store().await;

    let authentication = match store.sessions.get(&input.session_id) {
        Some(Session { authentication, .. }) => authentication,
        None => return Output::SessionNotFound,
    };

    let target_user = match store.users.get_by_id(&input.user_id) {
        Some(user) => user,
        None => return Output::UserNotFound,
    };

    if !permission::user_read(authentication, &*target_user.read().await).await {
        return Output::AccessDenied;
    };

    let target_user = target_user.read().await;

    return Output::Success {
        id: input.user_id,
        name: target_user.name.clone(),
        handle: target_user.handle.clone(),
    };
}
