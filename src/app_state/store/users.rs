use std::collections::hash_map;

use async_std::sync::Arc;
use async_std::sync::RwLock;
use async_std::sync::Weak;

use crate::lib::id::Id;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Id,
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
    IdExists,
    HandleExists,
}
#[derive(Debug)]
pub enum UpdateError {
    IdNotFound,
}

#[derive(Clone, Debug, Default)]
pub struct Users {
    by_id: hash_map::HashMap<Id, Arc<RwLock<User>>>,
    by_handle: hash_map::HashMap<String, Weak<RwLock<User>>>,
}

impl Users {
    pub fn insert(&mut self, user: User) -> Result<Arc<RwLock<User>>, InsertionError> {
        let handle_entry = match self.by_handle.entry(user.handle.clone()) {
            hash_map::Entry::Occupied(_) => {
                return Err(InsertionError::HandleExists);
            }
            hash_map::Entry::Vacant(entry) => entry,
        };

        let id_entry = match self.by_id.entry(user.id.clone()) {
            hash_map::Entry::Occupied(_) => {
                return Err(InsertionError::IdExists);
            }
            hash_map::Entry::Vacant(entry) => entry,
        };

        let user = Arc::new(RwLock::new(user));
        handle_entry.insert(Arc::downgrade(&user));
        id_entry.insert(user.clone());

        Ok(user)
    }

    pub fn get_by_handle(&self, handle: &str) -> Option<Arc<RwLock<User>>> {
        match self.by_handle.get(handle).map(Weak::upgrade) {
            Some(s) => s,
            None => None,
        }
    }

    pub fn get_by_id(&self, user_id: &Id) -> Option<Arc<RwLock<User>>> {
        self.by_id.get(user_id).cloned()
    }

    pub async fn update(
        &self,
        user_id: &Id,
        name: String,
        handle: String,
    ) -> Result<(), UpdateError> {
        let user_locked = self.get_by_id(&user_id).ok_or(UpdateError::IdNotFound)?;

        let mut user = user_locked.write().await;
        user.name = name;
        user.handle = handle;

        Ok(())
    }

    pub async fn update_password(&self, user_id: Id, password: String) -> Result<(), UpdateError> {
        let user_locked = self.get_by_id(&user_id).ok_or(UpdateError::IdNotFound)?;

        let mut user = user_locked.write().await;
        user.password = password;

        Ok(())
    }
}
