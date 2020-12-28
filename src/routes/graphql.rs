use crate::{app_state::LockedAppState, schema};
use schema::Schema;
use tide::{Request, Response};

pub async fn handle(mut request: Request<LockedAppState>) -> tide::Result<impl Into<Response>> {
    let query: juniper::http::GraphQLRequest = request.body_json().await?;
    let state = request.state();

    let schema = Schema::new(
        schema::QueryRoot(state.clone()),
        schema::MutationRoot(state.clone()),
    );
    let result = query.execute(&schema, &());
    Ok(Response::builder(200)
        .body(tide::Body::from_json(&result)?)
        .content_type(tide::http::mime::JSON))
}
