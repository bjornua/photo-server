use crate::{app_state::AppState, types::session::Session};
use serde_json;
use tide::{Request, Response};

pub async fn handle(req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let mut state = req.state().write();
    let session = state.new_session();
    return Ok(serde_json::to_value(Session::from(session)).unwrap());
}
