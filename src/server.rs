use crate::schema;

use tide::prelude::*;
use tide::{Request, Response};
#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Clone)]
struct State {
    schema: std::sync::Arc<schema::Schema>,
}

pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let state = State {
        schema: std::sync::Arc::new(schema::create_schema()),
    };
    let mut app = tide::with_state(state);
    app.at("/graphql").get(handle_graphiql);
    app.at("/graphql").post(handle_graphql);
    app.listen(socket).await?;
    Ok(())
}

async fn handle_graphiql(_req: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(juniper::graphiql::graphiql_source("/graphql"))
        .content_type(tide::http::mime::HTML))
}

async fn handle_graphql(mut request: Request<State>) -> tide::Result<impl Into<Response>> {
    let query: juniper::http::GraphQLRequest = request.body_json().await?;
    let result = query.execute(&request.state().schema, &());
    Ok(Response::builder(200)
        .body(tide::Body::from_json(&result)?)
        .content_type(tide::http::mime::JSON))
}
