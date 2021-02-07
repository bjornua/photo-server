pub mod event;
pub mod sessions;
pub mod store;
pub mod users;

use async_std::sync::{Arc, RwLock, RwLockReadGuard};
use event::Event;
use users::User;

use crate::app_state::event::DateEvent;

#[derive(Clone, Debug)]
pub struct Store {
    pub users: users::Users,
    pub sessions: sessions::Sessions,
}

impl Store {
    pub fn new() -> Self {
        Self {
            sessions: sessions::Sessions::new(),
            users: users::Users::new(),
        }
    }
}

impl Store {
    fn on_event(&mut self, command: DateEvent) {
        match command.kind {
            Event::SessionLogin {
                session_id,
                user_id,
            } => {
                let user = self.users.get_by_id(&user_id).unwrap();
                self.sessions.login(&session_id, Arc::downgrade(&user));
            }
            Event::SessionPing { session_id } => self.sessions.ping(&session_id, command.date),
            Event::SessionLogout { session_id } => self.sessions.logout(&session_id),
            Event::SessionCreate { session_id } => self.sessions.create(session_id, command.date),
            Event::UserCreate {
                id,
                name,
                handle,
                password,
            } => {
                self.users
                    .insert(User {
                        id,
                        name,
                        handle,
                        password,
                    })
                    .unwrap();
            }
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    store: Arc<RwLock<Store>>,
}

impl AppState {
    pub fn new() -> Self {
        return Self {
            store: Arc::new(RwLock::new(Store::new())),
        };
    }
}

impl AppState {
    pub async fn get_store<'a>(&'a self) -> RwLockReadGuard<'a, Store> {
        self.store.read().await
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write(self, undated_event: Event) -> Self {
        let event = DateEvent {
            date: chrono::Utc::now(),
            kind: undated_event,
        };
        println!("Logging event: {:?}", event);
        self.store.write().await.on_event(event);
        self
    }
}
