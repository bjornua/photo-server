use crate::{app_state::AppState, lib::authentication::get_session_id};

use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Deserialize)]
struct LogoutRequest {}

#[derive(Serialize)]
#[serde(tag = "type")]
enum LogoutResponse {
    Success,
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let session_id = get_session_id(&req).unwrap();
    let _: LogoutRequest = req.take_body().into_json().await?;

    req.state().write().await.logout(&session_id);

    return serde_json::to_value(LogoutResponse::Success).map_err(|e| tide::Error::new(422, e));
}
