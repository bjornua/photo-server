use crate::lib::file;
use crate::lib::id::Id;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Upload {
    pub id: Id,
    pub user_id: Id,
    pub size: u64,
    pub type_: file::Type,
    pub date_uploaded: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct Uploads {
    inner: HashMap<Id, Upload>,
}

impl Uploads {
    pub fn create(&mut self, upload: Upload) {
        let entry = self.inner.entry(upload.id.clone());
        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Upload exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => e.insert(upload),
        };
    }

    pub fn get(&self, session_id: &Id) -> Option<&Upload> {
        self.inner.get(session_id)
    }
}
