use crate::{
    app_state::AppState,
    lib::{authentication::get_session_id, authentication::Authentication},
};

use get_session_id::get_session_id;
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
    Failed,
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let session_id = get_session_id(&req).unwrap();
    let params: AuthRequest = req.take_body().into_json().await?;

    let authentication = req
        .state()
        .write()
        .login(&session_id, &params.username, &params.password);

    let result = match authentication {
        Authentication::NotAuthenticated => AuthResponse::Failed,
        Authentication::Authenticated { user: _ } => AuthResponse::Success,
    };

    return serde_json::to_value(result).map_err(|e| tide::Error::new(422, e));
}
