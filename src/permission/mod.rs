use crate::{app_state::users::User, lib::authentication::Authentication};
use async_std::sync::Arc;

fn get_auth_user(auth: &Authentication) -> Option<Arc<User>> {
    match auth {
        Authentication::NotAuthenticated => return None,
        Authentication::Authenticated { user } => user.upgrade(),
    }
}

pub fn session_list(auth: &Authentication) -> bool {
    return match get_auth_user(auth) {
        Some(_) => true,
        None => false,
    };
}

pub fn user_read(auth: &Authentication, target_user: &User) -> bool {
    let user = match get_auth_user(auth) {
        Some(user) => user,
        None => return false,
    };

    return *user == *target_user;
}

pub fn user_update(auth: &Authentication, target_user: &User) -> bool {
    let user = match get_auth_user(auth) {
        Some(user) => user,
        None => return false,
    };

    return *user == *target_user;
}
