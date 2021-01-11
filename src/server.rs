use crate::app_state::AppState;
use crate::routes;
pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let state: AppState = AppState::new();
    let mut app = tide::with_state(state);
    app.at("/authenticate").post(routes::authenticate::handle);
    app.at("/session/list").get(routes::session_list::handle);
    app.at("/session/create")
        .post(routes::session_create::handle);
    app.at("/upload").post(routes::upload::handle);
    app.listen(socket).await?;
    Ok(())
}
