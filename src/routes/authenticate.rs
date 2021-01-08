use crate::app_state::LockedAppState;

use serde;
use tide::{Request, Response};

#[derive(serde::Deserialize)]
struct Params {
    username: String,
    password: String,
}

pub async fn handle(mut req: Request<LockedAppState>) -> tide::Result<impl Into<Response>> {
    let params: Params = req.take_body().into_json().await.unwrap();

    return Ok(format!("{}", params.username));
}
