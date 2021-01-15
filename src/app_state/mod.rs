pub mod sessions;
pub mod users;

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use sessions::Session;

use crate::lib::{authentication::Authentication, id::ID};

#[derive(Clone, Debug)]
struct InnerAppState {
    pub users: users::Users,
    pub sessions: sessions::Sessions,
}

impl InnerAppState {
    pub fn new() -> Self {
        return Self {
            users: users::Users::new(),
            sessions: sessions::Sessions::new(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    inner: std::sync::Arc<RwLock<InnerAppState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: std::sync::Arc::new(RwLock::new(InnerAppState::new())),
        }
    }

    pub fn write(&self) -> WriteableState {
        WriteableState {
            inner: self.inner.write().unwrap(),
        }
    }

    pub fn read(&self) -> ReadableState {
        ReadableState {
            inner: self.inner.read().unwrap(),
        }
    }
}

pub struct ReadableState<'a> {
    inner: RwLockReadGuard<'a, InnerAppState>,
}

impl<'a> ReadableState<'a> {
    pub fn list_sessions(&self) -> Vec<&Session> {
        return self.inner.sessions.list();
    }
}

pub struct WriteableState<'a> {
    inner: RwLockWriteGuard<'a, InnerAppState>,
}

impl<'a> WriteableState<'a> {
    pub fn new_session(&mut self) -> &Session {
        let session = self.inner.sessions.create();
        return session;
    }

    pub fn login(&mut self, session_id: &ID, username: &str, password: &str) -> Authentication {
        let authentication = self.inner.users.authenticate(username, password);
        match authentication {
            a @ Authentication::NotAuthenticated => return a,
            Authentication::Authenticated { user } => {
                self.inner.sessions.login(session_id, user.clone());
                return Authentication::Authenticated { user };
            }
        }
    }

    pub fn logout(&mut self, session_id: &ID) -> () {
        self.inner.sessions.logout(session_id);
    }
}
