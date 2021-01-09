use crate::app_state;
use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub user: Option<User>,
}

impl From<&app_state::Session> for Session {
    fn from(s: &app_state::Session) -> Self {
        return Self {
            id: s.token.to_string(),
            user: s.user.as_ref().map(|u| u.into()),
        };
    }
}