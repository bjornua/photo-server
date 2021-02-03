use crate::lib::id::ID;

pub enum Command {
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

pub struct DatedCommand {
    pub date: chrono::DateTime<chrono::Utc>,
    pub kind: Command,
}
