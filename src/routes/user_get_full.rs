use crate::{
    app_state::AppState,
    lib::{authentication::get_authentication, id::ID},
};
use crate::{permission, types::user::User};
use serde::{Deserialize, Serialize};
use serde_json;
use tide::{Request, Response};

#[derive(Deserialize)]
struct UserRequest {
    user_id: ID,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum UserResponse {
    Success(User),
    Failure(Error),
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Error {
    AccessDenied,
    NotLoggedIn,

    NotFound,
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let params: UserRequest = req.take_body().into_json().await?;

    let state = req.state().read().await;
    let auth_user = get_authentication(&req, &state);

    let target_user = match state.get_user(&params.user_id) {
        Some(user) => user,
        None => {
            return Ok(serde_json::to_value(Error::NotFound).unwrap());
        }
    };

    if !permission::full_user_read(auth_user, &*target_user) {
        return Ok(serde_json::to_value(Error::AccessDenied).unwrap());
    };

    return Ok(serde_json::to_value(User::from(&*target_user)).unwrap());
}
