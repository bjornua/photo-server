use std::convert::TryFrom;

use async_std::path::PathBuf;

use crate::app_state::event::DateEvent;
use crate::app_state::event::Event;
use crate::app_state::log;

use crate::routes;

use crate::{app_state::AppState, lib::id::Id};

const LOG_FILE: &str = "thelog.log";

pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let log_writer =
        crate::app_state::log::file::Writer::new(&PathBuf::try_from(LOG_FILE).unwrap()).await;
    let log_reader = log::file::Reader::new(&PathBuf::try_from(LOG_FILE).unwrap()).await;
    let mut state = AppState::new(log_writer);
    state = state.replay(log_reader).await;

    if state
        .get_store()
        .await
        .users
        .get_by_handle("admin")
        .is_none()
    {
        state = state
            .write(DateEvent {
                date: chrono::Utc::now(),
                kind: Event::UserCreate {
                    user_id: Id::new(),
                    name: "Admin User".to_string(),
                    handle: "admin".to_string(),
                    password: "admin".to_string(),
                },
            })
            .await;
    }

    let app = make_app(state);
    app.listen(socket).await?;
    Ok(())
}

pub fn make_app<L: log::Writer + 'static>(state: AppState<L>) -> tide::Server<AppState<L>> {
    let mut app = tide::with_state(state);
    app.at("/command").post(routes::command::handle);
    app.at("/upload").post(routes::upload::handle);
    app
}
