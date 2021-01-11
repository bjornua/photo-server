use crate::{
    app_state::AppState,
    lib::{authentication::Authentication, id::ID},
};

use serde::{Deserialize, Serialize};
use tide::{http::Headers, Request, Response};

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AuthResponse {
    Success,
    Failed,
}

fn get_session_id<H: AsRef<Headers>>(headers: H) -> Option<ID> {
    let headers_ref = headers.as_ref();

    let value = headers_ref.get("Authorization")?.as_str();
    let mut words = value.splitn(2, " ");

    if words.next()? != "Bearer" {
        return None;
    };
    let str = words.next()?;

    return str.parse().ok();
}

pub async fn handle(mut req: Request<AppState>) -> tide::Result<impl Into<Response>> {
    let session_id = get_session_id(&req).unwrap();
    let params: AuthRequest = req.take_body().into_json().await?;

    let authentication =
        req.state()
            .write()
            .authenticate(&session_id, &params.username, &params.password);

    let result = match authentication {
        Authentication::NotAuthenticated => AuthResponse::Failed,
        Authentication::Authenticated { user: _ } => AuthResponse::Success,
    };

    return serde_json::to_value(result).map_err(|e| tide::Error::new(422, e));
}

#[cfg(test)]
mod tests {
    use crate::lib::id::ID;
    use tide::http::Url;

    #[test]
    fn test_get_session_id() {
        let url = Url::parse("http://example.org/").unwrap();
        let mut request = tide::http::Request::new(tide::http::Method::Post, url);

        request.insert_header("Authorization", "Bearer 3wB4St9NzSaC4r6ouj56eyRku22n");

        assert_eq!(
            super::get_session_id(request),
            "3wB4St9NzSaC4r6ouj56eyRku22n".parse::<ID>().ok()
        );
    }
}
