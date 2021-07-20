use pin_project::pin_project;
pub mod file;
pub mod memory;

use crate::lib::id::Id;
use async_std::io;

pub enum Blobs {
    Memory(memory::Blobs),
    File(file::Blobs),
}
pub enum BlobsReadError {
    Memory(memory::BlobsReadError),
    File(file::BlobsReadError),
}
pub enum BlobsNewBlobError {
    File(file::BlobsNewBlobError),
}
pub enum BlobsInsertError {
    File(file::BlobsInsertError),
}
pub enum BlobsDeleteError {
    File(file::BlobsDeleteError),
    Memory(memory::BlobsDeleteError),
}

impl Blobs {
    async fn read(&self, id: Id) -> Result<BlobReader, BlobsReadError> {
        match &self {
            Blobs::Memory(blobs) => blobs
                .read(id)
                .await
                .map(BlobReader::Memory)
                .map_err(BlobsReadError::Memory),
            Blobs::File(blobs) => blobs
                .read(id)
                .await
                .map(BlobReader::File)
                .map_err(BlobsReadError::File),
        }
    }

    pub async fn insert(&mut self, blob_writer: BlobWriter) -> Result<Id, BlobsInsertError> {
        match (self, blob_writer) {
            (Blobs::Memory(blobs), BlobWriter::Memory(writer)) => Ok(blobs.insert(writer).await),
            (Blobs::File(blobs), BlobWriter::File(writer)) => {
                blobs.insert(writer).await.map_err(BlobsInsertError::File)
            }
            _ => unreachable!(),
        }
    }

    pub async fn new_blob(&self) -> Result<BlobWriter, BlobsNewBlobError> {
        match &self {
            Blobs::Memory(blobs) => Ok(BlobWriter::Memory(blobs.new_blob().await)),
            Blobs::File(blobs) => blobs
                .new_blob()
                .await
                .map(BlobWriter::File)
                .map_err(BlobsNewBlobError::File),
        }
    }
    async fn delete(&mut self, id: Id) -> Result<(), BlobsDeleteError> {
        match self {
            Blobs::Memory(blobs) => blobs.delete(id).await.map_err(BlobsDeleteError::Memory),
            Blobs::File(blobs) => blobs.delete(id).await.map_err(BlobsDeleteError::File),
        }
    }
}

#[pin_project(project = LogReaderProj)]
pub enum BlobReader {
    Memory(#[pin] memory::BlobReader),
    File(#[pin] file::BlobReader),
}

impl io::Read for BlobReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match self.project() {
            LogReaderProj::Memory(reader) => reader.poll_read(cx, buf),
            LogReaderProj::File(reader) => reader.poll_read(cx, buf),
        }
    }
}

#[pin_project(project = LogWriterProj)]
pub enum BlobWriter {
    Memory(#[pin] memory::BlobWriter),
    File(#[pin] file::BlobWriter),
}

impl io::Write for BlobWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        match self.project() {
            LogWriterProj::Memory(writer) => writer.poll_write(cx, buf),
            LogWriterProj::File(writer) => writer.poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        match self.project() {
            LogWriterProj::Memory(writer) => writer.poll_flush(cx),
            LogWriterProj::File(writer) => writer.poll_flush(cx),
        }
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        match self.project() {
            LogWriterProj::Memory(writer) => writer.poll_close(cx),
            LogWriterProj::File(writer) => writer.poll_close(cx),
        }
    }
}
