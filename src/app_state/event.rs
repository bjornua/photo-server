use crate::lib::id::ID;

#[derive(Debug)]
pub enum Event {
    SessionLogin {
        session_id: ID,
        user_id: ID,
    },
    SessionPing {
        session_id: ID,
    },
    SessionLogout {
        session_id: ID,
    },
    SessionCreate {
        session_id: ID,
    },
    UserCreate {
        id: ID,
        name: String,
        handle: String,
        password: String,
    },
}
#[derive(Debug)]
pub struct DateEvent {
    pub date: chrono::DateTime<chrono::Utc>,
    pub kind: Event,
}
