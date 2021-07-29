use crate::lib::file;
use crate::lib::id::Id;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Upload {
    Uploading {
        id: Id,
        file_type: file::Type,
        date: chrono::DateTime<chrono::Utc>,
    },
    Ready {
        id: Id,
        file_type: file::Type,
        date: chrono::DateTime<chrono::Utc>,
        size: u64,
        blob_id: Id,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Uploads {
    inner: HashMap<Id, Upload>,
}

impl Uploads {
    pub fn upload_start(
        &mut self,
        id: Id,
        file_type: file::Type,
        date: chrono::DateTime<chrono::Utc>,
    ) {
        let entry = self.inner.entry(id.clone());
        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Upload exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => e.insert(Upload::Uploading {
                id,
                file_type,
                date,
            }),
        };
    }

    pub fn upload_finish(&mut self, file_id: Id, blob_id: Id, size: u64) {
        let entry = self.inner.get_mut(&file_id).unwrap();
        *entry = match entry {
            Upload::Uploading {
                id,
                file_type,
                date,
            } => Upload::Ready {
                id: id.clone(),
                file_type: file_type.clone(),
                date: date.clone(),
                size,
                blob_id,
            },
            _ => panic!("Upload is already finished"),
        };
    }

    pub fn get(&self, id: &Id) -> Option<&Upload> {
        self.inner.get(id)
    }
}
