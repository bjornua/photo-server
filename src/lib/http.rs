use tide::http::Headers;
use tide::http::Mime;

use crate::lib::file;

pub fn encode_response(result: impl serde::Serialize) -> tide::Result<tide::Response> {
    match serde_json::to_value(result) {
        Ok(value) => Ok(tide::Response::from(value)),
        Err(err) => {
            println!("Error serializing response: {}", err);
            Ok(tide::Response::new(tide::StatusCode::InternalServerError))
        }
    }
}

pub fn get_bearer_token(headers: &Headers) -> Option<&str> {
    let value = headers.get("Authorization")?.as_str();
    let mut words = value.splitn(2, ' ');

    if words.next()? != "Bearer" {
        return None;
    };

    words.next()
}

pub fn get_file_type(headers: &Headers) -> Option<file::Type> {
    let contenttype = headers.get("Content-Type")?.as_str();
    let mime_type = contenttype.parse::<Mime>().ok()?;

    match mime_type.essence() {
        "image/jpeg" => Some(file::Type::Jpg),
        "image/png" => Some(file::Type::Png),
        _ => None,
    }
}
