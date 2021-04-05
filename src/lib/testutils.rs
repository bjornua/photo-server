#[cfg(test)]
use crate::app_state::log;

#[cfg(test)]
use crate::app_state::AppRequest;

#[cfg(test)]
use crate::app_state::event::Event;

#[cfg(test)]
use crate::app_state::AppState;

#[cfg(test)]
use crate::lib::id::Id;

#[cfg(test)]
use std::str::FromStr;

#[cfg(test)]
pub async fn base_state() -> AppState<log::null::Writer> {
    let app_state = AppState::new(log::null::Writer {});
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
