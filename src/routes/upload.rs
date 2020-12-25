use crate::app_state::State;
use tide::{Request, Response};

pub async fn handle(_: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok(format!("Hello testing"))
}
