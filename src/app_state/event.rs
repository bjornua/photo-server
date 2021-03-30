use crate::lib::id::Id;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Event {
    SessionLogin {
        session_id: Id,
        user_id: Id,
    },
    SessionPing {
        session_id: Id,
    },
    SessionLogout {
        session_id: Id,
    },
    SessionCreate {
        session_id: Id,
    },
    UserCreate {
        user_id: Id,
        name: String,
        handle: String,
        password: String,
    },
    UserUpdate {
        user_id: Id,
        name: String,
        handle: String,
    },
    UserUpdatePassword {
        user_id: Id,
        password: String,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateEvent {
    pub date: chrono::DateTime<chrono::Utc>,
    pub kind: Event,
}
