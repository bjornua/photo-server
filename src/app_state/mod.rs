pub mod event;
pub mod log;
pub mod store;

use async_std::sync::Arc;
use async_std::sync::Mutex;
use async_std::sync::RwLock;
use async_std::sync::RwLockReadGuard;
use event::Event;
use std::path::PathBuf;

use crate::app_state::event::DateEvent;

pub struct AppState<L: log::Writer> {
    upload_dir: Arc<PathBuf>,
    store: Arc<RwLock<store::Store>>,
    logger: Arc<Mutex<L>>,
}

impl<L: log::Writer> Clone for AppState<L> {
    fn clone(&self) -> Self {
        Self {
            upload_dir: self.upload_dir.clone(),
            store: self.store.clone(),
            logger: self.logger.clone(),
        }
    }
}

impl<L: log::Writer> AppState<L> {
    pub fn new(logger: L, upload_dir: PathBuf) -> Self {
        Self {
            upload_dir: Arc::new(upload_dir),
            store: Arc::new(RwLock::new(store::Store::new())),
            logger: Arc::new(Mutex::new(logger)),
        }
    }

    pub fn into_app_request(self, date: chrono::DateTime<chrono::Utc>) -> AppRequestStore<L> {
        AppRequestStore {
            app_state: self,
            date,
        }
    }

    pub fn into_request_state_current_time(self) -> AppRequestStore<L> {
        self.into_app_request(chrono::Utc::now())
    }

    pub async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, store::Store> {
        self.store.read().await
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write_unlogged(self, event: DateEvent) -> Self {
        println!("{date}: {kind:?}", date = event.date, kind = event.kind);
        self.store.write().await.on_event(event).await;
        self
    }

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
pub struct AppRequestStore<L: log::Writer> {
    app_state: AppState<L>,
    date: chrono::DateTime<chrono::Utc>,
}

#[async_trait::async_trait]
pub trait AppRequest {
    async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, store::Store>;
    async fn write(mut self, event: Event) -> Self;
}

#[async_trait::async_trait]
impl<L: log::Writer> AppRequest for AppRequestStore<L> {
    async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, store::Store> {
        self.app_state.get_store().await
    }

    async fn write(mut self, event: Event) -> Self {
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
