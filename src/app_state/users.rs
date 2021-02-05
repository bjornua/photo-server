use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use async_std::sync::{Arc, RwLock, Weak};

use crate::lib::{authentication::Authentication, id::ID};

#[derive(Clone, Debug)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub handle: String,
    pub password: String,
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
pub enum InsertionError {
    IDExists,
    HandleExists,
}

#[derive(Clone, Debug)]
pub struct Users {
    by_id: HashMap<ID, Arc<RwLock<User>>>,
    by_handle: HashMap<String, Weak<RwLock<User>>>,
}

impl Users {
    pub fn new() -> Self {
        return Self {
            by_id: HashMap::new(),
            by_handle: HashMap::new(),
        };
    }

    pub fn insert(self, user: User) -> Result<Arc<RwLock<User>>, InsertionError> {
        let handle_entry = match self.by_handle.entry(user.handle.clone()) {
            Occupied(_) => {
                return Err(InsertionError::HandleExists);
            }
            Vacant(entry) => entry,
        };

        let id_entry = match self.by_id.entry(user.id.clone()) {
            Occupied(_) => {
                return Err(InsertionError::IDExists);
            }
            Vacant(entry) => entry,
        };

        let user = Arc::new(RwLock::new(user));
        handle_entry.insert(Arc::downgrade(&user));
        id_entry.insert(user.clone());

        return Ok(user);
    }

    pub fn get_by_handle(&self, handle: &str) -> Option<Arc<RwLock<User>>> {
        match self.by_handle.get(handle).map(Weak::upgrade) {
            Some(s) => return s,
            None => None,
        }
    }

    pub fn get_by_id(&self, user_id: &ID) -> Option<Arc<RwLock<User>>> {
        self.by_id.get(user_id).cloned()
    }
}
