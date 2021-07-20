use crate::lib::id::Id;
use async_std::io;
use async_std::io::Cursor;
use async_std::sync::RwLock;
use std::collections::HashMap;
use std::pin::Pin;

enum WriteError {}

#[derive(Default)]
pub struct Blobs {
    blobs: RwLock<HashMap<Id, Vec<u8>>>,
}

#[derive(Debug)]
pub enum BlobsReadError {
    NotFound,
}
#[derive(Debug)]
pub enum BlobsDeleteError {
    NotFound,
}

impl Blobs {
    pub async fn read(&self, id: Id) -> Result<BlobReader, BlobsReadError> {
        self.blobs
            .read()
            .await
            .get(&id)
            .map(|vec| BlobReader {
                blob: Cursor::new(vec.clone()),
            })
            .ok_or(BlobsReadError::NotFound)
    }

    pub async fn insert(&self, blob_writer: BlobWriter) -> Id {
        let id = Id::new();
        self.blobs
            .write()
            .await
            .insert(id.clone(), blob_writer.blob.into_inner());
        id
    }

    pub async fn new_blob(&self) -> BlobWriter {
        BlobWriter {
            blob: Cursor::new(Vec::new()),
        }
    }

    pub async fn delete(&self, id: Id) -> Result<(), BlobsDeleteError> {
        self.blobs
            .write()
            .await
            .remove(&id)
            .map(|_| ())
            .ok_or(BlobsDeleteError::NotFound)
    }
}

pub struct BlobReader {
    blob: Cursor<Vec<u8>>,
}
pub struct BlobWriter {
    blob: Cursor<Vec<u8>>,
}

impl io::Read for BlobReader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.blob).poll_read(cx, buf)
    }
}

impl io::Write for BlobWriter {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        Pin::new(&mut self.blob).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        Pin::new(&mut self.blob).poll_flush(cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        Pin::new(&mut self.blob).poll_close(cx)
    }
}
