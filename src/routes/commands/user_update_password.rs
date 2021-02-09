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
    pub password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Output {
    Success,
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

    if !permission::user_update_password(authentication, &*target_user.read().await).await {
        return Output::AccessDenied;
    };

    drop(store);

    state
        .write(crate::app_state::event::Event::UserUpdatePassword {
            user_id: input.user_id,
            password: input.password,
        })
        .await;

    return Output::Success;
}
