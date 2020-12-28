use crate::app_state::LockedAppState;
use tide::{Request, Response};

pub async fn handle(_req: Request<LockedAppState>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(juniper::graphiql::graphiql_source("/graphql"))
        .content_type(tide::http::mime::HTML))
}
