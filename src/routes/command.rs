use crate::{
    lib::command::Context,
    routes::commands::{login, logout, session_create, session_list, user_get_full},
};

use crate::{app_state::AppState, lib::authentication::get_session_id};

use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "args")]
enum Input {
    Login(login::Input),
    Logout(logout::Input),
    SessionCreate(session_create::Input),
    SessionList(session_list::Input),
    UserGetFull(user_get_full::Input),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "args")]
enum Output {
    Login(login::Output),
    Logout(logout::Output),
    SessionCreate(session_create::Output),
    SessionList(session_list::Output),
    UserGetFull(user_get_full::Output),
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let context = Context {
        session_id: get_session_id(&req),
        state: req.state(),
    };

    let command_input: Input = req.take_body().into_json().await?;

    let result: Output = match command_input {
        Input::Login(args) => Output::Login(login::run(context, args).await),
        Input::Logout(args) => Output::Logout(logout::run(context, args)),
        Input::SessionCreate(args) => Output::SessionCreate(session_create::run(context, args)),
        Input::SessionList(args) => Output::SessionList(session_list::run(context, args)),
        Input::UserGetFull(args) => Output::UserGetFull(user_get_full::run(context, args)),
    };

    return serde_json::to_value(result).map_err(|e| tide::Error::new(422, e));
}
