use crate::routes::commands;

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
    SessionList(commands::session_list::Input),
    UserGetFull(commands::user_get_full::Input),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "args")]
enum Output {
    Login(commands::login::Output),
    Logout(commands::logout::Output),
    SessionCreate(commands::session_create::Output),
    SessionList(commands::session_list::Output),
    UserGetFull(commands::user_get_full::Output),
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
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

    let result: Output = match command_input {
        Input::Login(args) => Output::Login(commands::login::run(state, args).await),
        Input::Logout(args) => Output::Logout(commands::logout::run(state, args).await),
        Input::SessionCreate(args) => {
            Output::SessionCreate(commands::session_create::run(state, args).await)
        }
        Input::SessionList(args) => {
            Output::SessionList(commands::session_list::run(state, args).await)
        }
        Input::UserGetFull(args) => {
            Output::UserGetFull(commands::user_get_full::run(state, args).await)
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
