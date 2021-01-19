use crate::app_state::AppState;
use crate::routes;
pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let state: AppState = AppState::new();
    let mut app = tide::with_state(state);
    app.at("/command").post(routes::command::handle);
    app.listen(socket).await?;
    Ok(())
}
