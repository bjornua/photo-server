use super::types;
use crate::{app_state::LockedAppState, timeline};
use juniper;

#[derive(Debug)]
pub struct QueryRoot(pub LockedAppState);

#[juniper::object]
impl QueryRoot {
    fn photos() -> Vec<types::photo::Photo> {
        timeline::get_pictures()
            .into_iter()
            .filter_map(|p| p.to_str().map(|p| p.to_string()))
            .map(|p| types::photo::Photo {
                id: p,
                hash: String::from("Test Hash"),
                width: 123.0,
                height: 123.0,
                date: chrono::MIN_DATETIME,
                size: 123.0,
                added: chrono::MIN_DATETIME,
            })
            .collect()
    }

    fn sessions(&self) -> Vec<types::session::Session> {
        let app_state = self.0 .0.read().unwrap();
        app_state
            .sessions
            .values()
            .map(|session| session.into())
            .collect()
    }
}
