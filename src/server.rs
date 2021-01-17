use crate::app_state::AppState;
use crate::routes;
pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let state: AppState = AppState::new();
    let mut app = tide::with_state(state);
    app.at("/login").post(routes::login::handle);
    app.at("/logout").post(routes::logout::handle);
    app.at("/user").post(routes::user_get_full::handle);
    app.at("/session/list").get(routes::session_list::handle);
    app.at("/session/create")
        .post(routes::session_create::handle);
    app.at("/upload").get(routes::upload::handle);
    app.listen(socket).await?;
    Ok(())
}
