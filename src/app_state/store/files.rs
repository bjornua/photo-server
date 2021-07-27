use crate::lib::file;
use crate::lib::id::Id;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum File {
    Waiting {
        id: Id,
        file_type: file::Type,
        maximum_size: u64,
        date: chrono::DateTime<chrono::Utc>,
    },
    Uploading {
        id: Id,
        file_type: file::Type,
        maximum_size: u64,
        date: chrono::DateTime<chrono::Utc>,
    },
    Ready {
        id: Id,
        size: u64,
        file_type: file::Type,
        blob_id: Id,
    },
}
#[derive(Debug, Clone, Default)]
pub struct Files {
    inner: HashMap<Id, File>,
}

impl Files {
    pub fn upload_new(
        &mut self,
        id: Id,
        file_type: file::Type,
        maximum_size: u64,
        date: chrono::DateTime<chrono::Utc>,
    ) {
        let entry = self.inner.entry(id.clone());
        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("File exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => e.insert(File::Waiting {
                id,
                file_type,
                maximum_size,
                date,
            }),
        };
    }

    pub fn upload_start(&mut self, id: Id) {
        let entry = self.inner.get_mut(&id).unwrap();
        *entry = match entry {
            File::Waiting {
                id,
                file_type,
                maximum_size,
                date,
            } => File::Uploading {
                id: id.clone(),
                file_type: file_type.clone(),
                maximum_size: *maximum_size,
                date: *date,
            },
            _ => panic!("Upload is not waiting"),
        };
    }

    pub fn upload_finish(&mut self, file_id: Id, blob_id: Id, size: u64) {
        let entry = self.inner.get_mut(&file_id).unwrap();
        *entry = match entry {
            File::Uploading {
                id,
                file_type,
                maximum_size: _,
                date: _,
            } => File::Ready {
                id: id.clone(),
                file_type: file_type.clone(),
                blob_id,
                size,
            },
            _ => panic!("Upload is already finished"),
        };
    }

    pub fn get(&self, id: &Id) -> Option<&File> {
        self.inner.get(id)
    }
}
