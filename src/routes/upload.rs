use crate::app_state::AppState;
use tide::{Request, Response};

pub async fn handle(_: Request<AppState>) -> tide::Result<impl Into<Response>> {
    Ok(format!("Hello testing"))
}
