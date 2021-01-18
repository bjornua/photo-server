use crate::app_state::LoginError;
use crate::lib::command::Context;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
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

pub async fn run<'a>(context: Context<'a>, input: Input) -> Output {
    let session_id = match context.session_id {
        Some(session_id) => session_id,
        None => return Output::InvalidSessionId,
    };

    let authentication =
        context
            .state
            .write()
            .await
            .login(&session_id, &input.handle, &input.password);

    return match authentication {
        Ok(()) => Output::Success,
        Err(LoginError::AuthenticationFailed) => Output::AuthenticationFailed,
        Err(LoginError::SessionNotFound) => Output::SessionNotFound,
    };
}
