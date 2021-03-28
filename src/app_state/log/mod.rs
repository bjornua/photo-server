use crate::app_state::event::DateEvent;
use async_trait::async_trait;

pub mod file;

#[async_trait]
pub trait Writer: Send {
    async fn write(&mut self, event: &DateEvent) -> ();
}
