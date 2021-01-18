use crate::app_state::AppState;

use super::id::ID;

pub struct Context<'a> {
    pub state: &'a AppState,
    pub session_id: Option<ID>,
}
