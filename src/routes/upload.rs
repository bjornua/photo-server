use crate::app_state::event::Event;
use crate::app_state::AppState;
use crate::lib::authentication::get_user;
use crate::lib::http;
use crate::lib::http::encode_response;
use crate::lib::id::Id;
use async_std::io::copy;

use serde::Deserialize;
use serde::Serialize;
use tide::Request;
use tide::Response;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Output {
    Success { upload_id: Id },
    NotAuthenticated,
    InvalidMimeType,
    InternalServerError,
}

pub async fn handle(req: Request<AppState>) -> tide::Result<Response> {
    encode_response(handle_inner(req).await)
}

pub async fn handle_inner(mut req: Request<AppState>) -> Output {
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

    let mut body = req.take_body();

    let blobs = state.get_blobs();
    let mut blob = blobs.new_blob().await.unwrap();

    let file_size = match copy(&mut body, &mut blob).await {
        Ok(size) => size,
        Err(e) => {
            println!("Body to file copy error: {:?}", e);
            return Output::InternalServerError;
        }
    };
    drop(store);

    let blob_id = blobs.insert(blob).await.unwrap();

    state
        .write(Event::UploadCreated {
            user_id,
            type_: file_type,
            size: file_size,
            upload_id: blob_id.clone(),
        })
        .await;

    Output::Success { upload_id: blob_id }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tide::http::Method;
    use tide::http::Request;
    use tide::http::Url;

    use crate::lib::id::Id;
    use crate::lib::testutils::base_state;

    use super::Output;

    #[async_std::test]
    async fn test_run_success() {
        let app_state = base_state().await;

        let state = app_state.clone().into_request_state_current_time();

        let app = crate::server::make_app(app_state);

        let mut request = Request::new(
            Method::Post,
            Url::parse("http://example.org/upload").unwrap(),
        );

        request.insert_header("Authorization", "Bearer 3zCD548f6YU7163rZ84ZGamWkQM");
        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();

        let upload_id = match result {
            Output::Success { upload_id } => upload_id,
            o => {
                panic!("Unexpected output {:?}", o);
            }
        };

        let store = state.get_store().await;
        let upload = store.uploads.get(&upload_id).unwrap();

        assert_eq!(upload.type_, crate::lib::file::Type::Jpg);
        assert_eq!(
            upload.user_id,
            Id::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap()
        );
        assert_eq!(upload.size, 0);
    }
}
