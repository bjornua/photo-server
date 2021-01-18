use crate::app_state::AppState;
use crate::types::session::Session;
use serde_json;
use tide::{Request, Response};

pub async fn handle(req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let sessions: Vec<Session> = req
        .state()
        .read()
        .await
        .list_sessions()
        .into_iter()
        .map(|session| session.into())
        .collect();

    return Ok(serde_json::to_value(sessions).unwrap());
}
