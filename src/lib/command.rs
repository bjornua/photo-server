use crate::app_state::AppState;

use super::id::Id;

pub struct Context<'a> {
    pub state: &'a AppState,
    pub session_id: Option<ID>,
}
