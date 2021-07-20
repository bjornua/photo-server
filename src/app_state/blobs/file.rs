use pin_project::pin_project;
use std::path;

use async_std::fs;
use async_std::io;

use crate::lib::id::Id;

enum WriteError {}

#[derive(Default)]
pub struct Blobs {
    blob_path: path::PathBuf,
    tmp_path: path::PathBuf,
}

#[derive(Debug)]
pub enum BlobsReadError {
    OpenError(io::Error),
}
#[derive(Debug)]
pub enum BlobsNewBlobError {
    OpenError(io::Error),
}
#[derive(Debug)]
pub enum BlobsDeleteError {
    DeleteError(io::Error),
}
#[derive(Debug)]
pub enum BlobsInsertError {
    OpenError(io::Error),
    RenameError(io::Error),
}

impl Blobs {
    pub async fn new(blob_path: path::PathBuf, tmp_path: path::PathBuf) -> Blobs {
        Blobs {
            blob_path,
            tmp_path,
        }
    }

    pub async fn read(&self, id: Id) -> Result<BlobReader, BlobsReadError> {
        let path = self.blob_path.clone().join(id.to_string());
        let file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(&path)
            .await
            .map_err(BlobsReadError::OpenError)?;
        Ok(BlobReader { id, file })
    }

    pub async fn insert(&self, blob_writer: BlobWriter) -> Result<Id, BlobsInsertError> {
        let id = Id::new();
        let path = self.blob_path.clone().join(id.to_string());

        // Just to make sure we are not overwriting something existing
        let _file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(&path)
            .await
            .map_err(BlobsInsertError::OpenError)?;

        fs::rename(blob_writer.path, path)
            .await
            .map_err(BlobsInsertError::RenameError)?;

        Ok(id)
    }

    pub async fn new_blob(&self) -> Result<BlobWriter, BlobsNewBlobError> {
        let id = Id::new();
        let path = self.tmp_path.clone().join(id.to_string());

        let file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(&path)
            .await
            .map_err(BlobsNewBlobError::OpenError)?;

        Ok(BlobWriter { path, file })
    }

    pub async fn delete(&self, id: Id) -> Result<(), BlobsDeleteError> {
        let path = self.blob_path.clone().join(id.to_string());
        fs::remove_file(path)
            .await
            .map_err(BlobsDeleteError::DeleteError)
    }
}

#[pin_project]
pub struct BlobWriter {
    path: path::PathBuf,
    #[pin]
    file: fs::File,
}

#[pin_project]
pub struct BlobReader {
    id: Id,
    #[pin]
    file: fs::File,
}

impl io::Read for BlobReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let this = self.project();
        this.file.poll_read(cx, buf)
    }
}

impl io::Write for BlobWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        let this = self.project();
        this.file.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = self.project();
        this.file.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = self.project();
        this.file.poll_close(cx)
    }
}
