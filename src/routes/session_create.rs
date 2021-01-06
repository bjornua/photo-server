use crate::app_state::LockedAppState;
use crate::types::session::Session;
use serde_json;
use tide::{Request, Response};

pub async fn handle(req: Request<LockedAppState>) -> tide::Result<impl Into<Response>> {
    let app_state = req.state();
    let mut app_state = app_state.0.write().unwrap();
    let session = crate::app_state::Session::new();
    let session = app_state
        .sessions
        .entry(session.token.clone())
        .or_insert(session);
    let session: Session = Session::from(&*session);
    return Ok(serde_json::to_value(session).unwrap());
}
