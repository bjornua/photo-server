use pin_project::pin_project;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::vec::IntoIter;

use crate::app_state::event::DateEvent;

/**
    Used for testing
*/

pub struct Log {
    events: Vec<DateEvent>,
}

impl Log {
    pub async fn new() -> Log {
        Log { events: Vec::new() }
    }

    pub async fn into_writer(self) -> Writer {
        Writer {
            events: self.events,
        }
    }

    pub async fn into_reader(self) -> Reader {
        Reader::new(self.events).await
    }
}

pub struct Writer {
    events: Vec<DateEvent>,
}

impl Writer {
    pub async fn write(&mut self, event: &DateEvent) {
        self.events.push(event.clone())
    }

    pub async fn into_log(self) -> Log {
        Log {
            events: self.events,
        }
    }
}

#[pin_project(project = ReaderProj)]
pub struct Reader {
    events: Vec<DateEvent>,
    iter: IntoIter<DateEvent>,
}

impl Reader {
    pub async fn new(events: Vec<DateEvent>) -> Reader {
        let iter = events.clone().into_iter();
        Reader { events, iter }
    }
    pub async fn into_log(self) -> Log {
        Log {
            events: self.events,
        }
    }
}

#[derive(Debug)]
pub enum StreamError {}
impl Reader {
    pub fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<DateEvent, StreamError>>> {
        let this = self.project();
        Poll::Ready(this.iter.next().map(Ok))
    }
}
