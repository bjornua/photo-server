use crate::app_state::{event::DateEvent, log};
use async_std::{
    fs::OpenOptions,
    io::{
        prelude::{BufReadExt, WriteExt},
        BufReader,
    },
};

pub struct Writer {
    file: async_std::fs::File,
}
impl Writer {
    pub async fn new(path: &async_std::path::Path) -> Self {
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
        dbg!(&serialized);

        self.file.write_all(serialized.as_bytes()).await.unwrap();
        self.file.write_all("\n".as_bytes()).await.unwrap();
        self.file.sync_all().await.unwrap();
    }
}

pub struct Reader {
    reader: async_std::io::BufReader<async_std::fs::File>,
    buffer: String,
}

impl Reader {
    pub async fn new(path: &async_std::path::Path) -> Self {
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
