use std::convert::TryFrom;

use async_std::path::PathBuf;

use crate::{
    app_state::{
        event::{DateEvent, Event},
        FileLogWriter,
    },
    routes,
};
use crate::{
    app_state::{AppState, FileLogReader},
    lib::id::ID,
};

const LOG_FILE: &str = "thelog.log";

pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let log_writer = FileLogWriter::new(&PathBuf::try_from(LOG_FILE).unwrap()).await;
    let log_reader = FileLogReader::new(&PathBuf::try_from(LOG_FILE).unwrap()).await;
    let mut state: AppState = AppState::new(log_writer);
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
                    user_id: ID::new(),
                    name: "Admin User".to_string(),
                    handle: "admin".to_string(),
                    password: "admin".to_string(),
                },
            })
            .await;
    }

    let mut app = tide::with_state(state);
    app.at("/command").post(routes::command::handle);
    app.listen(socket).await?;
    Ok(())
}
