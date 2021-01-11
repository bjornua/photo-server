use crate::types::user::User;
use crate::{app_state, lib::authentication::Authentication};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub user: Option<User>,
}

impl From<&app_state::sessions::Session> for Session {
    fn from(s: &app_state::sessions::Session) -> Self {
        return Self {
            id: s.token.to_string(),
            user: match &s.authentication {
                Authentication::NotAuthenticated => None,
                Authentication::Authenticated { ref user } => {
                    Some((&*user.upgrade().clone().unwrap()).into())
                }
            },
        };
    }
}
