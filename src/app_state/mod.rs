pub mod event;
pub mod sessions;
pub mod store;
pub mod users;

use async_std::sync::{Arc, RwLock, RwLockReadGuard};
use event::Event;
use users::User;

use crate::app_state::event::DateEvent;

trait Logger {
    fn test() -> () {}
}

struct FileLogger {}

impl Logger for FileLogger {}

struct NullLogger {}

impl Logger for FileLogger {}

#[derive(Clone, Debug)]
pub struct Store {
    pub users: users::Users,
    pub sessions: sessions::Sessions,
}

impl<L: Logger> Store<L> {
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

#[derive(Clone)]
pub struct AppState<L: Logger> {
    store: Arc<RwLock<Store>>,
    logger: L,
}

impl<L> AppState<L> {
    pub fn new(logger: L) -> Self {
        Self {
            store: Arc::new(RwLock::new(Store::new())),
            logger,
        }
    }

    pub fn into_request_state(self, date: chrono::DateTime<chrono::Utc>) -> RequestState {
        RequestState {
            app_state: self,
            date,
        }
    }

    pub fn into_request_state_current_time(self) -> RequestState {
        self.into_request_state(chrono::Utc::now())
    }

    pub async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, Store> {
        self.store.read().await
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write(self, event: DateEvent) -> Self {
        println!("{date}: {kind:?}", date = event.date, kind = event.kind);
        self.store.write().await.on_event(event).await;
        self
    }
}

#[derive(Clone)]
pub struct RequestState {
    app_state: AppState,
    date: chrono::DateTime<chrono::Utc>,
}

impl RequestState {
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

/*
    server
        create store
        replay log

        run server
            command
                append to log
                run command
*/
