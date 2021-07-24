use std::str::FromStr;

use crate::app_state::event::Event;
use crate::app_state::store::files;
use crate::app_state::AppState;

use crate::lib::http;
use crate::lib::http::encode_response;
use crate::lib::id::Id;
use async_std::io::copy;

use async_std::io::ReadExt;
use serde::Deserialize;
use serde::Serialize;
use tide::Request;
use tide::Response;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    NotAuthenticated,
    InvalidId,
    InvalidMimeType,
    InternalServerError,
    NotFound,
    AlreadyUploading,
    AlreadyUploaded,
    WrongFileType,
    TooLarge,
}

pub async fn handle(req: Request<AppState>) -> tide::Result<Response> {
    encode_response(handle_inner(req).await)
}

pub async fn handle_inner(mut req: Request<AppState>) -> Output {
    let state = req.state().clone().into_request_state_current_time();

    let file_id = match Id::from_str(req.param("file_id").unwrap()) {
        Ok(id) => id,
        Err(_) => return Output::InvalidId,
    };

    let (expected_file_type, maximum_size) = {
        match state.get_store().await.files.get(&file_id) {
            Some(files::File::Waiting {
                file_type,
                maximum_size,
                ..
            }) => (file_type.clone(), *maximum_size),
            Some(files::File::Uploading { .. }) => return Output::AlreadyUploading,
            Some(files::File::Ready { .. }) => return Output::AlreadyUploaded,
            None => return Output::NotFound,
        }
    };

    let state = state
        .write(Event::FileUploadStart {
            file_id: file_id.clone(),
        })
        .await;

    let file_type = match http::get_file_type(req.as_ref()) {
        Some(file_type) => file_type,
        None => return Output::InvalidMimeType,
    };

    if file_type != expected_file_type {
        return Output::WrongFileType;
    }

    let body = req.take_body();

    let blobs = state.get_blobs();
    let mut blob = blobs.new_blob().await.unwrap();

    // Read one byte more than max, so we can check it was exceeded
    let mut body = body.take(maximum_size + 1);

    let uploaded_size = match copy(&mut body, &mut blob).await {
        Ok(size) => size,
        Err(e) => {
            println!("Body to file copy error: {:?}", e);
            return Output::InternalServerError;
        }
    };

    if uploaded_size > maximum_size {
        return Output::TooLarge;
    }

    let blob_id = blobs.insert(blob).await.unwrap();

    state
        .write(Event::FileReady {
            file_id,
            blob_id,
            size: uploaded_size,
        })
        .await;

    Output::Success
}

#[cfg(test)]
mod tests {

    use crate::app_state::store::files::File;
    use crate::lib::id::Id;
    use crate::lib::testutils::base_state;

    use tide::http::Method;
    use tide::http::Request;
    use tide::http::Url;

    use super::Output;

    #[async_std::test]
    async fn test_run_success() {
        let app_state = base_state().await;

        let state = app_state.clone().into_request_state_current_time();

        let app = crate::server::make_app(app_state);
        let file_id = Id::new();

        let state = state
            .write(crate::app_state::event::Event::NewPhotoUpload {
                photo_id: Id::new(),
                file_id: file_id.clone(),
                file_type: crate::lib::file::Type::Jpg,
            })
            .await;

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();

        match result {
            Output::Success => (),
            o => {
                panic!("Unexpected output {:?}", o);
            }
        };

        let store = state.get_store().await;
        let upload = store.files.get(&file_id).unwrap();
        match upload {
            File::Waiting { .. } => panic!("Upload is waiting but should have finished"),
            File::Uploading { .. } => panic!("Upload is uploading but should have finished"),
            File::Ready { size, .. } => {
                assert_eq!(*size, 0);
            }
        }
    }

    #[async_std::test]
    async fn test_run_wrong_mime_type() {
        let app_state = base_state().await;

        let state = app_state.clone().into_request_state_current_time();

        let app = crate::server::make_app(app_state);
        let file_id = Id::new();

        state
            .write(crate::app_state::event::Event::NewPhotoUpload {
                photo_id: Id::new(),
                file_id: file_id.clone(),
                file_type: crate::lib::file::Type::Jpg,
            })
            .await;

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Content-Type", "image/png");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();

        assert_eq!(result, Output::WrongFileType);
    }

    #[async_std::test]
    async fn test_run_size_exceeded() {
        let app_state = base_state().await;

        let state = app_state.clone().into_request_state_current_time();

        let app = crate::server::make_app(app_state);
        let file_id = Id::new();

        state
            .write(crate::app_state::event::Event::NewPhotoUpload {
                photo_id: Id::new(),
                file_id: file_id.clone(),
                file_type: crate::lib::file::Type::Jpg,
            })
            .await;

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );
        request.set_body(" ".repeat(5_000_001));

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();

        assert_eq!(result, Output::TooLarge);
    }

    #[async_std::test]
    async fn test_run_max_size() {
        let app_state = base_state().await;

        let state = app_state.clone().into_request_state_current_time();

        let app = crate::server::make_app(app_state);
        let file_id = Id::new();

        state
            .write(crate::app_state::event::Event::NewPhotoUpload {
                photo_id: Id::new(),
                file_id: file_id.clone(),
                file_type: crate::lib::file::Type::Jpg,
            })
            .await;

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );
        request.set_body(" ".repeat(5_000_000));

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();

        assert_eq!(result, Output::Success);
    }
}
