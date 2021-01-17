use crate::{
    app_state::{AppState, LoginError},
    lib::authentication::get_session_id,
};

use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AuthResponse {
    Success,
    Failed { error: Error },
}
#[derive(Serialize)]
#[serde(tag = "type")]
enum Error {
    AuthenticationFailed,
    SessionNotFound,
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let session_id = get_session_id(&req).unwrap();
    let params: AuthRequest = req.take_body().into_json().await?;

    let authentication =
        req.state()
            .write()
            .await
            .login(&session_id, &params.username, &params.password);

    let result = match authentication {
        Ok(()) => AuthResponse::Success,
        Err(LoginError::AuthenticationFailed) => AuthResponse::Failed {
            error: Error::AuthenticationFailed,
        },
        Err(LoginError::SessionNotFound) => AuthResponse::Failed {
            error: Error::SessionNotFound,
        },
    };

    return serde_json::to_value(result).map_err(|e| tide::Error::new(422, e));
}
