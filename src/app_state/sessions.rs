use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Weak,
};

use crate::lib::{authentication::Authentication, id::ID};

use super::users::User;

#[derive(Clone, Debug)]
pub struct Session {
    pub token: ID,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub authentication: Authentication,
}

impl Session {
    pub fn new() -> Self {
        Self {
            token: ID::new(),
            authentication: Authentication::NotAuthenticated,
            last_seen: chrono::Utc::now(),
        }
    }
}

impl Hash for Session {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token.hash(state)
    }
}
impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.token.eq(&other.token)
    }
}
impl Eq for Session {}

#[derive(Clone, Debug)]
pub struct Sessions {
    inner: HashMap<ID, Session>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn create(&mut self) -> &Session {
        let session = Session::new();
        let session_ref = &session;
        let entry = self.inner.entry(session.token.clone());

        match entry {
            std::collections::hash_map::Entry::Occupied(e) => {
                panic!("Session exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => e.insert(session),
        }
    }

    pub fn list(&self) -> Vec<&Session> {
        self.inner.values().collect()
    }

    pub fn login(&mut self, sessionId: &ID, user: Weak<User>) -> Option<&Session> {
        let session = self.inner.get_mut(sessionId)?;
        session.authentication = Authentication::Authenticated { user };
        return Some(&*session);
    }
}
