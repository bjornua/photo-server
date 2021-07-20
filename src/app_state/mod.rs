pub mod blobs;
pub mod event;
pub mod log;
pub mod store;

use async_std::stream::StreamExt;
use async_std::sync::Arc;
use async_std::sync::Mutex;
use async_std::sync::RwLock;
use async_std::sync::RwLockReadGuard;
use event::Event;

use crate::app_state::event::DateEvent;
use crate::app_state::store::Store;

pub struct AppState {
    blobs: Arc<blobs::Blobs>,
    store: Arc<RwLock<store::Store>>,
    log_writer: Arc<Mutex<log::LogWriter>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            blobs: self.blobs.clone(),
            store: self.store.clone(),
            log_writer: self.log_writer.clone(),
        }
    }
}

impl AppState {
    pub fn new(logger: log::LogWriter, blobs: blobs::Blobs) -> Self {
        Self {
            blobs: Arc::new(blobs),
            store: Default::default(),
            log_writer: Arc::new(Mutex::new(logger)),
        }
    }
    pub fn new_with_store(store: Store, logger: log::LogWriter, blobs: blobs::Blobs) -> Self {
        Self {
            blobs: Arc::new(blobs),
            store: Arc::new(RwLock::new(store)),
            log_writer: Arc::new(Mutex::new(logger)),
        }
    }

    pub fn into_app_request(self, date: chrono::DateTime<chrono::Utc>) -> AppRequest {
        AppRequest {
            app_state: self,
            date,
        }
    }

    pub fn into_request_state_current_time(self) -> AppRequest {
        self.into_app_request(chrono::Utc::now())
    }

    pub async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, store::Store> {
        self.store.read().await
    }

    pub fn get_blobs(&self) -> Arc<blobs::Blobs> {
        self.blobs.clone()
    }

    // We take and return the value here to discourage deadlocks
    pub async fn write_unlogged(self, event: DateEvent) -> Self {
        println!("{date}: {kind:?}", date = event.date, kind = event.kind);
        self.store.write().await.on_event(event).await;
        self
    }

    pub async fn write(self, event: DateEvent) -> Self {
        self.log_writer.lock().await.write(&event).await;
        self.write_unlogged(event).await
    }
}

pub async fn create_store_from_log(reader: &mut log::LogReader) -> Store {
    let mut store = Store::default();
    while let Some(event) = reader.next().await {
        store.on_event(event.unwrap()).await;
    }
    store
}

#[derive(Clone)]
pub struct AppRequest {
    app_state: AppState,
    date: chrono::DateTime<chrono::Utc>,
}

impl AppRequest {
    pub async fn get_store<'a>(&self) -> RwLockReadGuard<'_, store::Store> {
        self.app_state.get_store().await
    }

    pub fn get_blobs(&self) -> Arc<blobs::Blobs> {
        self.app_state.get_blobs()
    }

    pub async fn write(mut self, event: Event) -> Self {
        self.app_state = self
            .app_state
            .write(DateEvent {
                date: self.date,
                kind: event,
            })
            .await;
        self
    }
}
