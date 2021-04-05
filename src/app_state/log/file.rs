use crate::app_state::event::DateEvent;
use crate::app_state::log;
use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::io::prelude::BufReadExt;
use async_std::io::prelude::WriteExt;
use async_std::io::BufReader;
use std::path::Path;
pub struct Writer {
    file: File,
}

impl Writer {
    pub async fn new(path: &Path) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .await
            .unwrap();
        return Self { file };
    }
}

#[async_trait::async_trait]
impl log::Writer for Writer {
    async fn write(&mut self, event: &DateEvent) {
        let serialized = serde_json::to_string(event).unwrap();

        self.file.write_all(serialized.as_bytes()).await.unwrap();
        self.file.write_all("\n".as_bytes()).await.unwrap();
        self.file.sync_all().await.unwrap();
    }
}

pub struct Reader {
    reader: BufReader<File>,
    buffer: String,
}

impl Reader {
    pub async fn new(path: &Path) -> Self {
        let file = OpenOptions::new().read(true).open(path).await.unwrap();
        let reader = BufReader::new(file);
        let buffer = String::new();
        Self { reader, buffer }
    }

    pub async fn next(&mut self) -> Option<DateEvent> {
        self.reader.read_line(&mut self.buffer).await.unwrap();

        if self.buffer.is_empty() {
            return None;
        }
        let command = serde_json::from_str(&self.buffer).unwrap();
        self.buffer.clear();
        Some(command)
    }
}
