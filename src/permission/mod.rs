use crate::{app_state::users::User, lib::authentication::Authentication};
use async_std::sync::{Arc, RwLock};

fn get_auth_user(auth: &Authentication) -> Option<Arc<RwLock<User>>> {
    match auth {
        Authentication::NotAuthenticated => None,
        Authentication::Authenticated { user } => user.upgrade(),
    }
}

pub fn session_list(auth: &Authentication) -> bool {
    get_auth_user(auth).is_some()
}

pub async fn user_read(auth: &Authentication, target_user: &User) -> bool {
    let user = match get_auth_user(auth) {
        Some(user) => user,
        None => return false,
    };

    return *user.read().await == *target_user;
}

pub async fn user_update(auth: &Authentication, target_user: &User) -> bool {
    let user = match get_auth_user(auth) {
        Some(user) => user,
        None => return false,
    };

    return *user.read().await == *target_user;
}
