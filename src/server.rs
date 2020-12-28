use crate::app_state::LockedAppState;

pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let state: LockedAppState = LockedAppState::new();
    let mut app = tide::with_state(state);
    app.at("/graphql").get(crate::routes::graphiql::handle);
    app.at("/graphql").post(crate::routes::graphql::handle);
    app.at("/photo").post(crate::routes::upload::handle);
    app.listen(socket).await?;
    Ok(())
}
