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

    pub fn write(&self) -> RwLockWriteGuard<InnerAppState> {
        self.inner.write().unwrap()
    }

    pub fn read(&self) -> RwLockReadGuard<InnerAppState> {
        self.inner.read().unwrap()
    }

    pub fn new_session(&self) -> &Session {
        let mut state = self.write();
        let session = state.sessions.create();
        return session;
    }
    pub fn authenticate(&self, sessionId: &ID, username: &str, password: &str) -> Authentication {
        let state = self.read();
        let authentication = state.users.authenticate(username, password);
        match authentication {
            a @ Authentication::NotAuthenticated => return a,
            Authentication::Authenticated { user } => {
                state.sessions.login(sessionId, user);
            }
        }
        return authentication;
    }
    pub fn list_sessions(&self) -> Vec<&Session> {
        let state = self.read();
        return state.sessions.list();
    }
}
