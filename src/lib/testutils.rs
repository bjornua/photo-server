use crate::app_state::blobs;
use crate::app_state::event::Event;
use crate::app_state::log;
use crate::app_state::AppState;
use crate::lib::id::Id;
use std::str::FromStr;

pub async fn test_state() -> AppState {
    let log_writer = log::Log::Memory(log::memory::Log::new().await);
    let blobs = blobs::Blobs::Memory(blobs::memory::Blobs::default());
    let app_state = AppState::new(log_writer.into_writer().await, blobs);
    app_state
}

pub async fn base_state() -> AppState {
    let app_state = test_state().await;

    let state = app_state.clone().into_request_state_current_time();
    state
        .write(Event::SessionCreate {
            session_id: Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
        })
        .await
        .write(Event::UserCreate {
            user_id: Id::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
            handle: "heidi".to_string(),
            name: "Heidi".to_string(),
            password: "eeQuee9t".to_string(),
        })
        .await
        .write(Event::SessionLogin {
            session_id: Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
            user_id: Id::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
        })
        .await;

    app_state
}
