use crate::schema;

use juniper::{EmptyMutation, FieldResult, Variables};
use tide::prelude::*;
use tide::Request;
#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Clone)]
struct State {}

pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let state = State {};
    let mut app = tide::with_state(state);
    app.at("/graphql").get(handle_graphiql);
    app.at("/graphql").post(handle_graphql);
    app.listen(socket).await?;
    Ok(())
}

async fn handle_graphiql(_: Request<State>) -> tide::Result<impl Into<tide::Response>> {
    Ok(tide::Response::builder(200)
        .body(juniper::graphiql::graphiql_source("/graphql"))
        .content_type(tide::http::mime::HTML))
}

async fn handle_graphql(mut request: Request<State>) -> tide::Result {
    let query = request.body_string().await?;
    let schema = schema::create_schema();
    let result = juniper::execute(&query, None, &schema, &Variables::new(), &());

    return match result {
        Ok((value, _errors)) => Ok(tide::Response::builder(tide::http::StatusCode::Ok)
            .body(tide::Body::from_json(&value)?)
            .build()),
        Err(e) => Ok(tide::Response::builder(tide::http::StatusCode::Ok)
            .body(tide::Body::from_json(&e)?)
            .build()),
    };
}

/*
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}
pub async fn run(socket: std::net::SocketAddr) -> io::Result<()> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt;

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

*/
