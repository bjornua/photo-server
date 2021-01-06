use crate::app_state::LockedAppState;
use tide::{Request, Response};

pub async fn handle(_: Request<LockedAppState>) -> tide::Result<impl Into<Response>> {
    Ok(format!("Hello testing"))
}
