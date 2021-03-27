use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Weak,
};

use async_std::sync::RwLock;

use crate::lib::{authentication::Authentication, id::Id};

use super::users::User;

#[derive(Clone, Debug)]
pub struct Session {
    pub token: Id,
    pub last_ping: chrono::DateTime<chrono::Utc>,
    pub authentication: Authentication,
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
    inner: HashMap<Id, Session>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn create(&mut self, token: Id, date: chrono::DateTime<chrono::Utc>) {
        let session = Session {
            authentication: Authentication::NotAuthenticated,
            last_ping: date,
            token,
        };
        let entry = self.inner.entry(session.token.clone());
        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Session exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => e.insert(session),
        };
    }

    pub fn get(&self, session_id: &Id) -> Option<&Session> {
        self.inner.get(session_id)
    }

    pub fn list(&self) -> Vec<&Session> {
        self.inner.values().collect()
    }

    pub fn login(&mut self, session_id: &Id, user: Weak<RwLock<User>>) -> Option<&Session> {
        let session = self.inner.get_mut(session_id)?;
        session.authentication = Authentication::Authenticated { user };
        Some(&*session)
    }

    pub fn ping(&mut self, session_id: &Id, date: chrono::DateTime<chrono::Utc>) {
        let session = self.inner.get_mut(session_id).unwrap();
        session.last_ping = date;
    }

    pub fn logout(&mut self, session_id: &Id) {
        let session = self.inner.get_mut(session_id).unwrap();
        session.authentication = Authentication::NotAuthenticated;
    }
}
