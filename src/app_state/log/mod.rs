use pin_project::pin_project;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use async_std::prelude::Stream;

use crate::app_state::event::DateEvent;

pub mod file;
pub mod memory;

pub enum Log {
    File(file::Log),
    Memory(memory::Log),
}

pub enum LogWriter {
    File(file::Writer),
    Memory(memory::Writer),
}

impl Log {
    pub async fn into_writer(self) -> LogWriter {
        match self {
            Log::File(log) => LogWriter::File(log.into_writer().await),
            Log::Memory(log) => LogWriter::Memory(log.into_writer().await),
        }
    }

    pub async fn into_reader(self) -> LogReader {
        match self {
            Log::File(log) => LogReader::File(log.into_reader().await),
            Log::Memory(log) => LogReader::Memory(log.into_reader().await),
        }
    }
}

impl LogWriter {
    pub async fn into_log(self) -> Log {
        match self {
            LogWriter::File(writer) => Log::File(writer.into_log().await),
            LogWriter::Memory(writer) => Log::Memory(writer.into_log().await),
        }
    }

    pub async fn write(&mut self, event: &DateEvent) {
        match self {
            LogWriter::File(writer) => writer.write(event).await,
            LogWriter::Memory(writer) => writer.write(event).await,
        }
    }
}

#[pin_project(project = LogReaderProj)]
pub enum LogReader {
    File(#[pin] file::Reader),
    Memory(#[pin] memory::Reader),
}

impl LogReader {
    pub async fn into_log(self) -> Log {
        match self {
            LogReader::File(reader) => Log::File(reader.into_log().await),
            LogReader::Memory(reader) => Log::Memory(reader.into_log().await),
        }
    }
}

#[derive(Debug)]
pub enum StreamError {
    FileStreamError(file::StreamError),
    MemoryStreamError(memory::StreamError),
}

impl Stream for LogReader {
    type Item = Result<DateEvent, StreamError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match this {
            LogReaderProj::File(reader) => {
                reader.poll_next(cx).map_err(StreamError::FileStreamError)
            }
            LogReaderProj::Memory(reader) => {
                reader.poll_next(cx).map_err(StreamError::MemoryStreamError)
            }
        }
    }
}
