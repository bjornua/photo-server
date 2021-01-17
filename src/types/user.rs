use crate::app_state;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
}

impl From<&app_state::users::User> for User {
    fn from(u: &app_state::users::User) -> Self {
        return Self {
            id: u.id.to_string(),
            username: u.username.to_string(),
        };
    }
}
