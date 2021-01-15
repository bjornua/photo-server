use crate::{app_state::users::User, lib::authentication::Authentication};
use std::sync::Arc;

fn get_auth_user(auth: Authentication) -> Option<Arc<User>> {
    match auth {
        Authentication::NotAuthenticated => return None,
        Authentication::Authenticated { user } => user.upgrade(),
    }
}

pub fn full_user_read(auth: Authentication, targetUser: &User) -> bool {
    let user = match get_auth_user(auth) {
        Some(user) => user,
        None => return false,
    };

    return *user == *targetUser;
}
