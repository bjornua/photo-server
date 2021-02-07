use crate::lib::id::ID;

#[derive(Debug, Clone)]
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
        user_id: ID,
        name: String,
        handle: String,
        password: String,
    },
}
#[derive(Debug, Clone)]
pub struct DateEvent {
    pub date: chrono::DateTime<chrono::Utc>,
    pub kind: Event,
}
