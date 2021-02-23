use std::future::Future;

use crate::app_state::event::DateEvent;

pub mod file;

trait Writer {
    fn write(&mut self, event: &DateEvent) -> Box<dyn Future<Output = ()>>;
}
