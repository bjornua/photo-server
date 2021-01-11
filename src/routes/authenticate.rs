use crate::{
    app_state::AppState,
    lib::{authentication::Authentication, id::ID},
};

use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AuthResponse {
    Success,
    Failed,
}

fn getSessionId<T>(req: &Request<T>) -> Option<ID> {
    let value = req.header("Authorization")?.as_str();

    let mut words = value.splitn(1, " ");

    if words.next()? != "Bearer" {
        return None;
    };

    return words.next()?.parse().ok();
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let sessionId = getSessionId(&req).unwrap();
    let params: AuthRequest = req.take_body().into_json().await?;

    let authentication = req
        .state()
        .authenticate(&sessionId, &params.username, &params.password);

    let result = match authentication {
        Authentication::NotAuthenticated => AuthResponse::Failed,
        Authentication::Authenticated { user } => AuthResponse::Success,
    };

    return serde_json::to_value(result).map_err(|e| tide::Error::new(422, e));
}
