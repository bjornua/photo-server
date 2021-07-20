use crate::app_state::store::sessions::Session;
use crate::app_state::store::users::User;
use crate::app_state::store::Store;
use crate::lib::http::get_bearer_token;
use crate::lib::id::Id;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use async_std::sync::Weak;
use tide::http::Headers;

#[derive(Clone, Debug)]
pub enum Authentication {
    NotAuthenticated,
    Authenticated { user: Weak<RwLock<User>> },
}

impl Authentication {
    pub fn get_user(&self) -> Option<Arc<RwLock<User>>> {
        match self {
            Authentication::NotAuthenticated => None,
            Authentication::Authenticated { user } => user.upgrade(),
        }
    }
}

pub fn get_session_id(headers: &Headers) -> Option<Id> {
    let token = get_bearer_token(headers)?;
    token.parse().ok()
}

pub fn get_authentication(headers: &Headers, store: &Store) -> Authentication {
    let session_id = match get_session_id(headers) {
        Some(session_id) => session_id,
        None => return Authentication::NotAuthenticated,
    };

    let session_maybe: Option<&Session> = store.sessions.get(&session_id);

    match session_maybe {
        Some(s) => s.authentication.clone(),
        None => Authentication::NotAuthenticated,
    }
}

pub async fn get_user(headers: &Headers, store: &Store) -> Option<Arc<RwLock<User>>> {
    get_authentication(headers, &*store).get_user()
}

#[cfg(test)]
mod tests {
    use crate::app_state::event::Event;
    use crate::app_state::AppRequest;
    use crate::lib::id::Id;
    use crate::lib::testutils;
    use std::str::FromStr;
    use tide::http::Url;

    #[test]
    fn test_get_session_id() {
        let url = Url::parse("http://example.org/").unwrap();
        let mut request = tide::http::Request::new(tide::http::Method::Post, url);

        request.insert_header("Authorization", "Bearer 3wB4St9NzSaC4r6ouj56eyRku22n");

        assert_eq!(
            super::get_session_id(request.as_ref()),
            "3wB4St9NzSaC4r6ouj56eyRku22n".parse::<Id>().ok()
        );
    }

    #[async_std::test]
    async fn test_get_user() {
        let url = Url::parse("http://example.org/").unwrap();
        let mut request = tide::http::Request::new(tide::http::Method::Post, url);

        request.insert_header("Authorization", "Bearer 3wB4St9NzSaC4r6ouj56eyRku22n");
        let state = testutils::test_state()
            .await
            .into_request_state_current_time();

        let state = state
            .write(Event::SessionCreate {
                session_id: Id::from_str("3wB4St9NzSaC4r6ouj56eyRku22n").unwrap(),
            })
            .await
            .write(Event::UserCreate {
                user_id: Id::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
                handle: "heidi".to_string(),
                name: "Heidi".to_string(),
                password: "eeQuee9t".to_string(),
            })
            .await
            .write(Event::SessionLogin {
                session_id: Id::from_str("3wB4St9NzSaC4r6ouj56eyRku22n").unwrap(),
                user_id: Id::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
            })
            .await;

        let store = state.get_store().await;
        let user_locked = super::get_user(request.as_ref(), &store).await.unwrap();
        let user = user_locked.read().await;

        assert_eq!(
            user.id,
            "2bQFgyUNCCRUs8SitkgBG8L37KL1".parse::<Id>().unwrap()
        );
    }
}
