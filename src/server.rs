use async_std::net::SocketAddr;

use crate::app_state;
use crate::app_state::blobs;
use crate::app_state::event::DateEvent;
use crate::app_state::event::Event;
use crate::app_state::log;
use crate::lib::config::Config;

use crate::routes;

use crate::app_state::AppState;
use crate::lib::id::Id;

const LOG_FILE: [&str; 1] = ["thelog.log"];

pub async fn run(config: Config) -> tide::Result<()> {
    let log = log::Log::File(log::file::Log::new(LOG_FILE.iter().collect()).await);
    let mut log_reader = log.into_reader().await;
    let store = app_state::create_store_from_log(&mut log_reader).await;
    let blobs = blobs::Blobs::File(blobs::file::Blobs::new(config.blobs_dir, config.tmp_dir).await);
    let log = log_reader.into_log().await;

    let mut state = AppState::new_with_store(store, log.into_writer().await, blobs);
    state = make_admin_user_if_needed(state).await;
    let app = make_app(state);
    let socket = SocketAddr::new(config.ip.into(), config.port);
    app.listen(socket).await?;
    Ok(())
}

async fn make_admin_user_if_needed(state: AppState) -> AppState {
    if state
        .get_store()
        .await
        .users
        .get_by_handle("admin")
        .is_none()
    {
        return state
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

    state
}

pub fn make_app(state: AppState) -> tide::Server<AppState> {
    let mut app = tide::with_state(state);
    app.at("/command").post(routes::command::handle);
    app.at("/upload").post(routes::upload::handle);
    app
}
