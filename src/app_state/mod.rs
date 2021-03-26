pub mod event;
pub mod log;
pub mod sessions;
pub mod store;
pub mod users;

use async_std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
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

    async fn on_event(&mut self, command: DateEvent) {
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
                user_id: id,
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
            Event::UserUpdate {
                user_id,
                name,
                handle,
            } => {
                self.users.update(&user_id, name, handle).await.unwrap();
            }
            Event::UserUpdatePassword { user_id, password } => {
                self.users.update_password(user_id, password).await.unwrap();
            }
        }
    }
}

pub struct AppState<L: log::Writer> {
    store: Arc<RwLock<Store>>,
    logger: Arc<Mutex<L>>,
}

impl<L: log::Writer> Clone for AppState<L> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            logger: self.logger.clone(),
        }
    }
}

impl<L: log::Writer> AppState<L> {
    pub fn new(logger: L) -> Self {
        Self {
            store: Arc::new(RwLock::new(Store::new())),
            logger: Arc::new(Mutex::new(logger)),
        }
    }

    pub fn into_request_state(self, date: chrono::DateTime<chrono::Utc>) -> RequestState<L> {
        RequestState {
            app_state: self,
            date,
        }
    }

    pub fn into_request_state_current_time(self) -> RequestState<L> {
        self.into_request_state(chrono::Utc::now())
    }

    pub async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, Store> {
        self.store.read().await
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write_unlogged(self, event: DateEvent) -> Self {
        println!("{date}: {kind:?}", date = event.date, kind = event.kind);
        self.store.write().await.on_event(event).await;
        self
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write(self, event: DateEvent) -> Self {
        self.logger.lock().await.write(&event).await;
        self.write_unlogged(event).await
    }

    // We take and return the value here to discourage deadlocks
    pub async fn replay(mut self, mut reader: log::file::Reader) -> Self {
        while let Some(test) = reader.next().await {
            self = self.write_unlogged(test).await;
        }
        self
    }
}

#[derive(Clone)]
pub struct RequestState<L: log::Writer> {
    app_state: AppState<L>,
    date: chrono::DateTime<chrono::Utc>,
}

impl<L: log::Writer> RequestState<L> {
    pub async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, Store> {
        self.app_state.get_store().await
    }

    pub async fn write(mut self, event: Event) -> Self {
        self.app_state = self
            .app_state
            .write(DateEvent {
                date: self.date,
                kind: event,
            })
            .await;
        return self;
    }
}
