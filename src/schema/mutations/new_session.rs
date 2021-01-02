use crate::{app_state::LockedAppState, schema::types::session::Session};

pub fn new_session(app_state: &LockedAppState) -> Session {
    let mut app_state = app_state.0.write().unwrap();
    let session = crate::app_state::Session::new();
    let session = app_state
        .sessions
        .entry(session.token.clone())
        .or_insert(session);
    return (&*session).into();
}
