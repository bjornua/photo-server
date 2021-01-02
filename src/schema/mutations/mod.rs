pub mod new_session;
use crate::{app_state::LockedAppState, schema::types::session::Session};
use juniper;

#[derive(Debug)]
pub struct MutationRoot(pub LockedAppState);

#[juniper::object]
impl MutationRoot {
    fn new_session(&self) -> Session {
        new_session::new_session(&self.0)
    }
}
