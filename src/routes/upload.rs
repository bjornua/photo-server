use crate::app_state::event::Event;
use crate::app_state::AppState;

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
    Success,
    NotAuthenticated,
    InvalidId,
    InvalidMimeType,
    FileUploadError,
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

    let file_type = match http::get_file_type(req.as_ref()) {
        Some(file_type) => file_type,
        None => return Output::InvalidMimeType,
    };

    let upload_id = Id::new();

    let state = state
        .write(Event::UploadBegin {
            upload_id: upload_id.clone(),
            file_type: file_type,
        })
        .await;

    let mut body = req.take_body();

    let blobs = state.get_blobs();
    let mut blob = blobs.new_blob().await.unwrap();

    let uploaded_size = match copy(&mut body, &mut blob).await {
        Ok(size) => size,
        Err(e) => {
            println!("Body to file copy error: {:?}", e);
            return Output::FileUploadError;
        }
    };

    let blob_id = blobs.insert(blob).await.unwrap();

    state
        .write(Event::UploadFinish {
            upload_id,
            blob_id,
            size: uploaded_size,
        })
        .await;

    Output::Success
}

#[cfg(test)]
mod tests {

    use std::pin::Pin;
    use std::str::FromStr;
    use std::task::Context;
    use std::task::Poll;

    use crate::app_state::store::files::Upload;
    use crate::lib::id::Id;
    use crate::lib::testutils::base_state;

    use async_std::io::BufReader;

    use tide::http::Method;
    use tide::http::Request;
    use tide::http::Url;
    use tide::Body;

    use super::Output;

    #[async_std::test]
    async fn test_run_success() {
        let app_state = base_state().await;

        let state = app_state.clone().into_request_state_current_time();

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Authorization", "Bearer 3zCD548f6YU7163rZ84ZGamWkQM");
        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();

        assert_eq!(result, Output::Success);

        let store = state.get_store().await;
        let upload = store.files.get(&file_id).unwrap();

        match upload {
            Upload::Uploading { .. } => panic!("Upload is uploading but should have finished"),
            Upload::Ready {
                size, file_type, ..
            } => {
                assert_eq!(*file_type, crate::lib::file::Type::Jpg);
                assert_eq!(*size, 0);
            }
        }
    }

    #[async_std::test]
    async fn test_run_wrong_mime_type() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

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

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

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

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

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

    #[async_std::test]
    async fn test_run_invalid_mime_type() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Content-Type", "asdf");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::InvalidMimeType);
    }

    #[async_std::test]
    async fn test_run_invalid_id() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/asdf")).unwrap(),
        );

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::InvalidId);
    }

    #[async_std::test]
    async fn test_run_non_existing() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::NotFound);
    }

    #[async_std::test]
    async fn test_run_already_uploaded() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::Success);

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );

        request.insert_header("Content-Type", "image/jpeg");

        let mut response: tide::http::Response = app.respond(request).await.unwrap();
        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::AlreadyUploaded);
    }

    #[async_std::test]
    async fn test_run_already_uploading() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

        // We don't want to actually finish the requests since this
        // is testing the behaviour of multiple requests
        struct PendingReader {}
        impl async_std::io::Read for PendingReader {
            fn poll_read(
                self: Pin<&mut Self>,
                _: &mut Context<'_>,
                _: &mut [u8],
            ) -> Poll<std::io::Result<usize>> {
                return Poll::Pending;
            }
        }

        let mut request0 = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );
        request0.insert_header("Content-Type", "image/jpeg");
        request0.set_body(Body::from_reader(BufReader::new(PendingReader {}), Some(4)));
        let response0 = app.respond(request0);

        let mut request1 = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );
        request1.insert_header("Content-Type", "image/jpeg");
        request1.set_body(Body::from_reader(BufReader::new(PendingReader {}), Some(4)));
        let response1 = app.respond(request1);

        let mut response: tide::http::Response = futures_lite::future::race(response0, response1)
            .await
            .unwrap();

        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::AlreadyUploading);
    }

    #[async_std::test]
    async fn test_run_failed_body() {
        let app_state = base_state().await;

        let app = crate::server::make_app(app_state);
        let file_id = Id::from_str("FzXiG9uZVyTPn4NRQ3HGV6B3eCK").unwrap();

        struct FailingReader {}
        impl async_std::io::Read for FailingReader {
            fn poll_read(
                self: Pin<&mut Self>,
                _: &mut Context<'_>,
                _: &mut [u8],
            ) -> Poll<std::io::Result<usize>> {
                return Poll::Ready(Err(async_std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    "",
                )));
            }
        }

        let mut request = Request::new(
            Method::Post,
            Url::parse(&format!("http://example.org/file/{}", file_id)).unwrap(),
        );
        request.insert_header("Content-Type", "image/jpeg");
        request.set_body(Body::from_reader(BufReader::new(FailingReader {}), None));
        let mut response: tide::http::Response = app.respond(request).await.unwrap();

        let result: Output = response.body_json().await.unwrap();
        assert_eq!(result, Output::FileUploadError);
    }
}
