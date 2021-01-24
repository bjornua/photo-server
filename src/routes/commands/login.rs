use crate::{
    app_state::{AppState, LoginError},
    lib::id::ID,
};
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
    let authentication = state
        .write()
        .await
        .login(&input.session_id, &input.handle, &input.password)
        .await;

    return match authentication {
        Ok(()) => Output::Success,
        Err(LoginError::AuthenticationFailed) => Output::AuthenticationFailed,
        Err(LoginError::SessionNotFound) => Output::SessionNotFound,
    };
}
