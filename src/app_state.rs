use crate::schema;
use std::collections::HashMap;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Session {
    pub id: String,
    pub user: Option<User>,
}

impl Session {
    pub fn new() -> Self {
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        println!("{}", rand_string);

        Self {
            id: rand_string,
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

#[derive(Clone, Debug)]
pub struct State {
    pub schema: std::sync::Arc<schema::Schema>,
    pub users: HashMap<String, User>,
    pub sessions: HashMap<String, Session>,
}

impl State {
    pub fn new() -> Self {
        Self {
            schema: std::sync::Arc::new(schema::create_schema()),
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}
