use std::borrow::Borrow;

use crate::{app_state::{self, AppState}, lib::{authentication::Authentication, id::ID}};
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
    InvalidSessionId,
}

pub async fn run<'a>(state: &AppState, input: Input) -> Output {
    let user_ref = match state.users.get_by_handle(&input.handle) {
        Some(user) => user,
        None => return Output::AuthenticationFailed,
    };
    let user = user_ref.read().await;
    let test = &mut user;

    

    if user.password != input.password {
        return Output::AuthenticationFailed;
    }


    return Authentication::Authenticated {
        user: Arc::downgrade(&user_ref),
    };

    AppState.
    let authentication = state
        .write()
        .await
        .login(&input.session_id, &input.handle, &input.password)
        .await;

    return match authentication {
        Ok(()) => Output::Success,
        Err(LoginError::AuthenticationFailed) => ,
        Err(LoginError::SessionNotFound) => ,
    };
}
