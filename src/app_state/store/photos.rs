use std::collections::HashMap;

use crate::lib::id::Id;

#[derive(Clone, Debug)]
pub struct Photo {
    photo_id: Id,
    file_id: Id,
}
#[derive(Clone, Debug)]
pub struct PhotoUploading {
    photo_id: Id,
    file_id: Id,
}

#[derive(Clone, Debug, Default)]
pub struct Photos {
    photos: HashMap<Id, Photo>,
    uploading: HashMap<Id, PhotoUploading>,
}

impl Photos {
    pub fn photo_new_upload(&mut self, photo_id: Id, file_id: Id) {
        let entry = self.uploading.entry(file_id.clone());
        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Upload already exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(PhotoUploading { photo_id, file_id })
            }
        };
    }

    pub fn photo_upload_finish(&mut self, file_id: Id) {
        let PhotoUploading { photo_id, file_id } = self.uploading.remove(&file_id).unwrap();

        let entry = self.photos.entry(photo_id.clone());
        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Photo already exists")
            }
            std::collections::hash_map::Entry::Vacant(e) => e.insert(Photo { photo_id, file_id }),
        };
    }
}
