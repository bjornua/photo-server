pub mod files;
pub mod photos;
pub mod sessions;
pub mod users;

use crate::app_state::event::Event;
use async_std::sync::Arc;

use crate::app_state::event::DateEvent;

#[derive(Clone, Debug, Default)]
pub struct Store {
    pub users: users::Users,
    pub sessions: sessions::Sessions,
    pub files: files::Files,
    pub photos: photos::Photos,
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
            Event::SessionPing { session_id } => {
                self.sessions.ping(&session_id, command.date);
            }
            Event::SessionLogout { session_id } => {
                self.sessions.logout(&session_id);
            }
            Event::SessionCreate { session_id } => {
                self.sessions.create(session_id, command.date);
            }
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
            Event::NewPhotoUpload {
                file_id,
                photo_id,
                file_type,
            } => {
                self.photos.photo_new_upload(photo_id, file_id.clone());
                self.files
                    .upload_new(file_id, file_type, 5_000_000, command.date);
            }
            Event::FileUploadStart { file_id } => {
                self.files.upload_start(file_id);
            }
            Event::FileReady {
                file_id,
                size,
                blob_id,
            } => {
                self.files.upload_finish(file_id.clone(), blob_id, size);
                self.photos.photo_upload_finish(file_id);
            }
        }
    }
}
