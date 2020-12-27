use crate::schema;
use std::collections::HashMap;

use bs58;
use rand::{thread_rng, Rng};

enum TokenDecodeError {
    BS58DecodeError(bs58::decode::Error),
    WrongLength,
}

#[derive(Clone, Debug)]
pub struct Token([u8; 32]);

impl Token {
    fn new() -> Token {
        Token(thread_rng().gen())
    }

    fn decode(s: String) -> Result<Token, TokenDecodeError> {
        let mut buffer: [u8; 32] = [0; 32];
        let size = bs58::decode(s)
            .into(&mut buffer)
            .map_err(TokenDecodeError::BS58DecodeError)?;

        if size != 32 {
            return Err(TokenDecodeError::WrongLength);
        }

        return Ok(Token(buffer));
    }

    fn encode(&self) -> String {
        bs58::encode(&self.0).into_string()
    }
}

#[derive(Clone, Debug)]
pub struct Session {
    pub token: Token,
    pub user: Option<User>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            token: Token::new(),
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
