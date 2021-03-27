pub mod event;
pub mod log;
pub mod store;

use async_std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use event::Event;

use crate::app_state::event::DateEvent;

pub struct AppState<L: log::Writer> {
    store: Arc<RwLock<store::Store>>,
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
            store: Arc::new(RwLock::new(store::Store::new())),
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

    pub async fn get_store<'a>(&'_ self) -> RwLockReadGuard<'_, store::Store> {
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

trait RequestState {
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
