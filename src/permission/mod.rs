use crate::{app_state::users::User, lib::authentication::Authentication};

pub fn session_list(auth: &Authentication) -> bool {
    auth.get_user().is_some()
}

pub async fn user_read(auth: &Authentication, target_user: &User) -> bool {
    let user = match auth.get_user() {
        Some(user) => user,
        None => return false,
    };

    return *user.read().await == *target_user;
}

pub async fn user_update(auth: &Authentication, target_user: &User) -> bool {
    let user = match auth.get_user() {
        Some(user) => user,
        None => return false,
    };

    return *user.read().await == *target_user;
}

pub async fn user_update_password(auth: &Authentication, target_user: &User) -> bool {
    let user = match auth.get_user() {
        Some(user) => user,
        None => return false,
    };

    return *user.read().await == *target_user;
}
