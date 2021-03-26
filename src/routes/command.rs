use crate::{app_state::log::Writer, routes::commands};

use crate::app_state::AppState;

use serde::{Deserialize, Serialize};
use tide::{Request, Response, StatusCode};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "args")]
enum Input {
    Login(commands::login::Input),
    Logout(commands::logout::Input),
    SessionCreate(commands::session_create::Input),
    SessionGet(commands::session_get::Input),
    SessionList(commands::session_list::Input),
    SessionPing(commands::session_ping::Input),
    UserGetFull(commands::user_get_full::Input),
    UserUpdate(commands::user_update::Input),
    UserUpdatePassword(commands::user_update_password::Input),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "result")]
enum Output {
    Login(commands::login::Output),
    Logout(commands::logout::Output),
    SessionCreate(commands::session_create::Output),
    SessionGet(commands::session_get::Output),
    SessionList(commands::session_list::Output),
    SessionPing(commands::session_ping::Output),
    UserGetFull(commands::user_get_full::Output),
    UserUpdate(commands::user_update::Output),
    UserUpdatePassword(commands::user_update_password::Output),
}

pub async fn handle<T: Writer>(mut req: Request<AppState<T>>) -> tide::Result<impl Into<Response>> {
    let command_input: Input = match req.take_body().into_json().await {
        Ok(input) => input,
        Err(err) => {
            let err = err.downcast::<serde_json::Error>();
            return Ok(match err {
                Ok(serde_err) => {
                    let mut response = Response::new(StatusCode::UnprocessableEntity);
                    response.set_body(serde_err.to_string());
                    response
                }
                Err(err) => {
                    println!("Error: {}", err);
                    Response::new(StatusCode::UnprocessableEntity)
                }
            });
        }
    };

    let state = req.state();
    let state = state.clone().into_request_state_current_time();

    let result: Output = match command_input {
        Input::Login(args) => Output::Login(commands::login::run(state, args).await),
        Input::Logout(args) => Output::Logout(commands::logout::run(state, args).await),
        Input::SessionCreate(args) => {
            Output::SessionCreate(commands::session_create::run(state, args).await)
        }
        Input::SessionList(args) => {
            Output::SessionList(commands::session_list::run(state, args).await)
        }
        Input::SessionGet(args) => {
            Output::SessionGet(commands::session_get::run(state, args).await)
        }
        Input::UserGetFull(args) => {
            Output::UserGetFull(commands::user_get_full::run(state, args).await)
        }
        Input::SessionPing(args) => {
            Output::SessionPing(commands::session_ping::run(state, args).await)
        }
        Input::UserUpdate(args) => {
            Output::UserUpdate(commands::user_update::run(state, args).await)
        }
        Input::UserUpdatePassword(args) => {
            Output::UserUpdatePassword(commands::user_update_password::run(state, args).await)
        }
    };

    return match serde_json::to_value(result) {
        Ok(value) => Ok(Response::from(value)),
        Err(err) => {
            println!("Error serializing response: {}", err);
            Ok(Response::new(StatusCode::UnprocessableEntity))
        }
    };
}
