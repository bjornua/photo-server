use tide::http::Headers;
use tide::http::Mime;

use crate::lib::file;

pub fn encode_response(result: impl serde::Serialize) -> tide::Result<tide::Response> {
    match serde_json::to_value(result) {
        Ok(value) => Ok(tide::Response::from(value).into()),
        Err(err) => {
            println!("Error serializing response: {}", err);
            Ok(tide::Response::new(tide::StatusCode::InternalServerError))
        }
    }
}

pub fn get_bearer_token<'a>(headers: &'a Headers) -> Option<&'a str> {
    let value = headers.get("Authorization")?.as_str();
    let mut words = value.splitn(2, ' ');

    if words.next()? != "Bearer" {
        return None;
    };

    return words.next();
}

pub fn get_file_type(headers_ref: &Headers) -> Option<file::Type> {
    let headers = headers_ref.as_ref();
    let contenttype = headers.get("Content-Type")?.as_str();
    let mime_type = contenttype.parse::<Mime>().ok()?;

    match mime_type.basetype() {
        "image/jpeg" => Some(file::Type::Jpg),
        "image/png" => Some(file::Type::Png),
        _ => None,
    }
}
