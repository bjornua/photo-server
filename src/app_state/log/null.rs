use crate::app_state::event::DateEvent;
use crate::app_state::log;
#[derive(Clone)]
pub struct Writer {}

#[async_trait::async_trait]
impl log::Writer for Writer {
    async fn write(&mut self, _event: &DateEvent) {}
}
