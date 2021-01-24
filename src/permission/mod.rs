use crate::{app_state::users::User, lib::authentication::Authentication};
use async_std::sync::{Arc, RwLock};

fn get_auth_user(auth: &Authentication) -> Option<Arc<RwLock<User>>> {
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
