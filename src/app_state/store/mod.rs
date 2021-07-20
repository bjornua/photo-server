pub mod sessions;
pub mod uploads;
pub mod users;

use crate::app_state::event::Event;
use crate::app_state::store::uploads::Upload;
use async_std::sync::Arc;

use crate::app_state::event::DateEvent;

#[derive(Clone, Debug, Default)]
pub struct Store {
    pub users: users::Users,
    pub sessions: sessions::Sessions,
    pub uploads: uploads::Uploads,
}

impl Store {
    pub async fn on_event(&mut self, command: DateEvent) {
        match command.kind {
            Event::SessionLogin {
                session_id,
                user_id,
            } => {
                let user = self.users.get_by_id(&user_id).unwrap();
                self.sessions.login(&session_id, Arc::downgrade(&user));
            }
            Event::SessionPing { session_id } => self.sessions.ping(&session_id, command.date),
            Event::SessionLogout { session_id } => self.sessions.logout(&session_id),
            Event::SessionCreate { session_id } => self.sessions.create(session_id, command.date),
            Event::UserCreate {
                user_id: id,
                name,
                handle,
                password,
            } => {
                self.users
                    .insert(users::User {
                        id,
                        name,
                        handle,
                        password,
                    })
                    .unwrap();
            }
            Event::UserUpdate {
                user_id,
                name,
                handle,
            } => {
                self.users.update(&user_id, name, handle).await.unwrap();
            }
            Event::UserUpdatePassword { user_id, password } => {
                self.users.update_password(user_id, password).await.unwrap();
            }
            Event::UploadCreated {
                user_id,
                upload_id: file_id,
                type_: file_type,
                size: file_size,
            } => self.uploads.create({
                Upload {
                    id: file_id,
                    user_id,
                    type_: file_type,
                    size: file_size,
                    date_uploaded: command.date,
                }
            }),
        }
    }
}
