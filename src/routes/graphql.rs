use crate::app_state::State;
use tide::{Request, Response};

pub async fn handle(mut request: Request<State>) -> tide::Result<impl Into<Response>> {
    let query: juniper::http::GraphQLRequest = request.body_json().await?;
    let result = query.execute(&request.state().schema, &());
    Ok(Response::builder(200)
        .body(tide::Body::from_json(&result)?)
        .content_type(tide::http::mime::JSON))
}
