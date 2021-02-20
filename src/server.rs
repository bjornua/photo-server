use std::convert::TryFrom;

use async_std::path::PathBuf;

use crate::{app_state::AppState, lib::id::ID};
use crate::{
    app_state::{
        event::{DateEvent, Event},
        FileLogger,
    },
    routes,
};

const LOG_FILE: &str = "thelog.log";

pub async fn run(socket: std::net::SocketAddr) -> tide::Result<()> {
    let logger: FileLogger = FileLogger::new(&PathBuf::try_from(LOG_FILE).unwrap()).await;
    let state: AppState = AppState::new(logger);
    let state = state
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

    let mut app = tide::with_state(state);
    app.at("/command").post(routes::command::handle);
    app.listen(socket).await?;
    Ok(())
}
