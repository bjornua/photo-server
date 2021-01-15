use crate::{
    app_state::AppState,
    lib::{authentication::get_authentication, id::ID},
};
use crate::{
    permission,
    types::{session::Session, user::User},
};
use serde::{Deserialize, Serialize};
use serde_json;
use tide::{Request, Response};

#[derive(Deserialize)]
struct UserRequest {
    userId: ID,
}
#[derive(Serialize)]
enum UserResponse {
    Success(User),
    Failure(Error),
}

#[derive(Serialize)]
enum Error {
    AccessDenied,
    NotFound,
}

pub async fn handle(req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let state = req.state().read();
    let auth_user = match get_user() {

    };

    let user = 

    let authentication = get_authentication(req, req.getState());

    if !permission::full_user_read(session.user, user) {};

    let sessions: Vec<Session> = req
        .state()
        .read()
        .get_user()
        .into_iter()
        .map(|session| session.into())
        .collect();

    return Ok(serde_json::to_value(sessions).unwrap());
}
