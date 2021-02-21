pub mod event;
pub mod sessions;
pub mod store;
pub mod users;

use async_std::{
    fs::OpenOptions,
    io::{
        prelude::{BufReadExt, WriteExt},
        BufReader,
    },
    sync::{Arc, Mutex, RwLock, RwLockReadGuard},
};
use event::Event;
use users::User;

use crate::app_state::event::DateEvent;

pub struct FileLogWriter {
    file: async_std::fs::File,
}

impl FileLogWriter {
    pub async fn new(path: &async_std::path::Path) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .await
            .unwrap();
        return Self { file };
    }

    pub async fn append(&mut self, event: &DateEvent) {
        let serialized = serde_json::to_string(event).unwrap();
        dbg!(&serialized);

        self.file.write_all(serialized.as_bytes()).await.unwrap();
        self.file.write_all("\n".as_bytes()).await.unwrap();
        self.file.sync_all().await.unwrap();
    }
}

pub struct FileLogReader {
    reader: async_std::io::BufReader<async_std::fs::File>,
    buffer: String,
}

impl FileLogReader {
    pub async fn new(path: &async_std::path::Path) -> Self {
        let file = OpenOptions::new().read(true).open(path).await.unwrap();
        let reader = BufReader::new(file);
        let buffer = String::new();
        Self { reader, buffer }
    }

    pub async fn get(&mut self) -> DateEvent {
        self.reader.read_line(&mut self.buffer).await.unwrap();

        dbg!(&self.buffer);
        let command = serde_json::from_str(&self.buffer).unwrap();
        self.buffer.clear();
        command
    }
}

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

#[derive(Clone)]
pub struct AppState {
    store: Arc<RwLock<Store>>,
    logger: Arc<Mutex<FileLogWriter>>,
}

impl AppState {
    pub fn new(logger: FileLogWriter) -> Self {
        Self {
            store: Arc::new(RwLock::new(Store::new())),
            logger: Arc::new(Mutex::new(logger)),
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
    pub async fn write_unlogged(self, event: DateEvent) -> Self {
        println!("{date}: {kind:?}", date = event.date, kind = event.kind);
        self.store.write().await.on_event(event).await;
        self
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write(self, event: DateEvent) -> Self {
        self.logger.lock().await.append(&event).await;
        self.write_unlogged(event).await
    }

    // We take and return the value here to discourage deadlocks
    pub async fn replay(mut self, mut reader: FileLogReader) -> Self {
        loop {
            let test = reader.get().await;
            self = self.write_unlogged(test).await;
        }
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
