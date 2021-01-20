pub mod sessions;
pub mod users;

use async_std::sync::Arc;
use async_std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use sessions::Session;
use users::User;

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
    inner: async_std::sync::Arc<RwLock<InnerAppState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: async_std::sync::Arc::new(RwLock::new(InnerAppState::new())),
        }
    }

    pub async fn write<'a>(&'a self) -> WriteableState<'a> {
        WriteableState {
            inner: self.inner.write().await,
        }
    }

    pub async fn read<'a>(&'a self) -> ReadableState<'a> {
        ReadableState {
            inner: self.inner.read().await,
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

    pub fn get_session(&self, session_id: &ID) -> Option<&Session> {
        self.inner.sessions.get(session_id)
    }

    pub fn get_user(&self, user_id: &ID) -> Option<Arc<User>> {
        self.inner.users.get(user_id)
    }
}

pub struct WriteableState<'a> {
    inner: RwLockWriteGuard<'a, InnerAppState>,
}

pub enum LoginError {
    SessionNotFound,
    AuthenticationFailed,
}

impl<'a> WriteableState<'a> {
    pub fn new_session(&mut self) -> &Session {
        let session = self.inner.sessions.create();
        return session;
    }

    pub fn login(
        &mut self,
        session_id: &ID,
        handle: &str,
        password: &str,
    ) -> Result<(), LoginError> {
        let authentication = self.inner.users.authenticate(handle, password);

        let session = self
            .inner
            .sessions
            .get_mut(session_id)
            .ok_or(LoginError::SessionNotFound)?;

        return match authentication {
            Authentication::NotAuthenticated => Err(LoginError::AuthenticationFailed),
            a @ Authentication::Authenticated { .. } => {
                session.authentication = a;
                Ok(())
            }
        };
    }

    pub fn logout(&mut self, session_id: &ID) -> Option<&Session> {
        self.inner.sessions.logout(session_id)
    }

    pub fn get_user_mut(&mut self, user_id: &ID) -> Option<&mut User> {
        self.inner.users.get_mut(user_id)
    }
}
