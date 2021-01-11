use crate::app_state::users::User;

#[derive(Clone, Debug)]
pub enum Authentication {
    NotAuthenticated,
    Authenticated { user: std::sync::Weak<User> },
}
