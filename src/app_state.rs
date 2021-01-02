use crate::lib::id::ID;

use std::{collections::HashMap, sync::RwLock};

#[derive(Clone, Debug)]
pub struct Session {
    pub token: ID,
    pub user: Option<User>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            token: ID::new(),
            user: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct LockedAppState(pub std::sync::Arc<RwLock<AppState>>);

impl LockedAppState {
    pub fn new() -> Self {
        Self(std::sync::Arc::new(RwLock::new(AppState::new())))
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub users: HashMap<ID, User>,
    pub sessions: HashMap<ID, Session>,
}

impl AppState {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        let admin = User {
            id: ID::new(),
            name: "Admin User".into(),
            password: "admin".into(),
            username: "admin".into(),
        };
        users.insert(admin.id.clone(), admin);

        let app_state = Self {
            users,
            sessions: HashMap::new(),
        };
        println!("{:?}", app_state);

        return app_state;
    }
}
