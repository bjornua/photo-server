use crate::app_state::AppState;
use tide::{Request, Response};

pub async fn handle(_: Request<AppState>) -> tide::Result<impl Into<Response>> {
    Ok("Hello testing".to_string())
}
