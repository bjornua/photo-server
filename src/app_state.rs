use crate::lib::token;

use std::{collections::HashMap, sync::RwLock};

#[derive(Clone, Debug)]
pub struct Session {
    pub token: token::Token,
    pub user: Option<User>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            token: token::Token::new(),
            user: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
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
    pub users: HashMap<String, User>,
    pub sessions: HashMap<token::Token, Session>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}
