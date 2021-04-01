use crate::app_state::AppState;
use crate::app_state::{event::Event, AppRequest};
use crate::lib::authentication::get_user;
use crate::lib::http;
use crate::lib::http::encode_response;
use crate::{app_state::log::Writer, lib::id::Id};
use async_std::io::copy;

use serde::Serialize;
use tide::{log, Request, Response};

#[derive(Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    NotAuthenticated,
    InvalidMimeType,
    InternalServerError,
}

pub async fn handle<T: Writer>(req: Request<AppState<T>>) -> tide::Result<Response> {
    encode_response(handle_inner(req).await)
}

pub async fn handle_inner<T: Writer>(mut req: Request<AppState<T>>) -> Output {
    let state = req.state().clone().into_request_state_current_time();

    let store = state.get_store().await;
    let user_id = match get_user(req.as_ref(), &store).await {
        Some(user) => user.read().await.id.clone(),
        None => return Output::NotAuthenticated,
    };

    let file_type = match http::get_file_type(req.as_ref()) {
        Some(file_type) => file_type,
        None => return Output::InvalidMimeType,
    };

    let body = req.take_body();

    let file_id = Id::new();
    let file_name = format!("./uploads/{}", file_id);

    let file = match async_std::fs::OpenOptions::new()
        .write(true)
        .read(false)
        .create_new(true)
        .open(file_name)
        .await
    {
        Ok(file) => file,
        Err(e) => {
            log::error!("Could not create file: {:?}", e);
            return Output::InternalServerError;
        }
    };

    let file_size = match copy(body, file).await {
        Ok(size) => size,
        Err(e) => {
            log::error!("Body to file copy error: {:?}", e);
            return Output::InternalServerError;
        }
    };
    drop(store);

    state.write(Event::UserFileUploaded {
        user_id,
        file_type,
        file_size,
        file_id,
    });

    return Output::Success;
}
