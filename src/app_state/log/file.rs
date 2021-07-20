use crate::app_state::event::DateEvent;
use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::io::prelude::BufReadExt;
use async_std::io::prelude::WriteExt;
use async_std::io::BufReader;
use async_std::io::Lines;
use async_std::stream::Stream;
use async_std::task::Context;
use async_std::task::Poll;
use pin_project::pin_project;
use std::path;

use std::pin::Pin;

pub struct Log {
    path: path::PathBuf,
}

impl Log {
    pub async fn new(path: path::PathBuf) -> Log {
        Log { path }
    }

    pub async fn into_writer(self) -> Writer {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .await
            .unwrap();

        Writer {
            file,
            path: self.path,
        }
    }

    pub async fn into_reader(self) -> Reader {
        Reader::new(self.path).await
    }
}

impl Reader {
    pub async fn new(path: path::PathBuf) -> Reader {
        let file = OpenOptions::new().read(true).open(&path).await.unwrap();
        let reader = BufReader::new(file);

        Reader {
            lines: reader.lines(),
            path,
        }
    }

    pub async fn into_log(self) -> Log {
        Log::new(self.path).await
    }
}

#[pin_project(project = LogReaderProj)]
pub struct Reader {
    path: path::PathBuf,
    #[pin]
    lines: Lines<BufReader<File>>,
}

#[derive(Debug)]
pub enum StreamError {}

impl Stream for Reader {
    type Item = Result<DateEvent, StreamError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.project().lines.poll_next(cx) {
            Poll::Ready(Some(Ok(line))) => {
                let command = serde_json::from_str(&line).unwrap();
                Poll::Ready(Some(Ok(command)))
            }
            Poll::Ready(Some(Err(_e))) => todo!(),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Writer {
    file: File,
    path: path::PathBuf,
}

impl Writer {
    pub async fn write(&mut self, event: &DateEvent) {
        let serialized = serde_json::to_string(event).unwrap();

        self.file.write_all(serialized.as_bytes()).await.unwrap();
        self.file.write_all("\n".as_bytes()).await.unwrap();
        self.file.sync_all().await.unwrap();
    }

    pub async fn into_log(self) -> Log {
        Log::new(self.path).await
    }
}
