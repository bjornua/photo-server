use crate::app_state::LockedAppState;
use crate::types::session::Session;
use serde_json;
use tide::{Request, Response};

pub async fn handle(req: Request<LockedAppState>) -> tide::Result<impl Into<Response>> {
    let state = req.state();
    let app_state = state.0.read().unwrap();
    let sessions: Vec<Session> = app_state
        .sessions
        .values()
        .map(|session| session.into())
        .collect();

    return Ok(serde_json::to_value(sessions).unwrap());
}
