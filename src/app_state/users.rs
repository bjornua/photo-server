use async_std::sync::Arc;

use crate::lib::{authentication::Authentication, id::ID};

#[derive(Clone, Debug)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub username: String,
    pub password: String,
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug)]
pub struct Users {
    inner: Vec<std::sync::Arc<User>>,
}

impl Users {
    pub fn new() -> Self {
        let admin = User {
            id: ID::new(),
            name: String::from("Admin User"),
            password: String::from("admin"),
            username: String::from("admin"),
        };

        Self {
            inner: vec![Arc::new(admin)],
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Authentication {
        let user = self.inner.iter().find(|&u| u.username == username);

        match user {
            Some(user) if user.password == password => Authentication::Authenticated {
                user: Arc::downgrade(user),
            },
            Some(_) | None => return Authentication::NotAuthenticated,
        }
    }

    pub fn get(&self, user_id: &ID) -> Option<Arc<User>> {
        self.inner.iter().find(|&s| s.id == *user_id).cloned()
    }
}
