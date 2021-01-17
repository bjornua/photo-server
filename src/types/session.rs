use crate::{app_state, lib::authentication::Authentication};
use crate::{lib::id::ID, types::user::User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: ID,
    pub user: Option<User>,
}

impl From<&app_state::sessions::Session> for Session {
    fn from(s: &app_state::sessions::Session) -> Self {
        return Self {
            id: s.token.clone(),
            user: match &s.authentication {
                Authentication::NotAuthenticated => None,
                Authentication::Authenticated { ref user } => {
                    Some((&*user.upgrade().clone().unwrap()).into())
                }
            },
        };
    }
}
