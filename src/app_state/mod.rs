pub mod command;
pub mod sessions;
pub mod store;
pub mod users;

use async_std::sync::Arc;
use command::Command;
use tide::Middleware;
use users::User;

use crate::app_state::command::DatedCommand;

#[derive(Clone, Debug)]
struct AppState {
    pub users: users::Users,
    pub sessions: sessions::Sessions,
}

impl AppState {
    fn on_command(&mut self, command: DatedCommand) {
        match command.kind {
            Command::SessionLogin {
                session_id,
                user_id,
            } => {
                let user = self.users.get_by_id(&session_id).unwrap();
                self.sessions.login(&session_id, Arc::downgrade(&user));
            }
            Command::SessionPing { session_id } => self.sessions.ping(&session_id, command.date),
            Command::SessionLogout { session_id } => self.sessions.logout(&session_id),
            Command::SessionCreate { session_id } => self.sessions.create(session_id, command.date),
            Command::UserCreate {
                id,
                name,
                handle,
                password,
            } => {
                self.users
                    .insert(User {
                        id,
                        name,
                        handle,
                        password,
                    })
                    .unwrap();
            }
        }
    }
}
