use crate::app_state::store::users::User;
// use app_state::{sessions::Session, Store};
use async_std::sync::{Arc, RwLock, Weak};

// use tide::http::Headers;

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

/*
pub fn get_session_id<H: AsRef<Headers>>(headers: H) -> Option<ID> {
    let headers_ref = headers.as_ref();

    let value = headers_ref.get("Authorization")?.as_str();
    let mut words = value.splitn(2, ' ');

    if words.next()? != "Bearer" {
        return None;
    };
    let str = words.next()?;

    str.parse().ok()
}

pub fn get_authentication<H: AsRef<Headers>>(headers: H, store: &Store) -> Authentication {
    let session_id = match get_session_id(headers) {
        Some(id) => id,
        None => return Authentication::NotAuthenticated,
    };

    let session_maybe: Option<&Session> = store.sessions.get(&session_id);

    match session_maybe {
        Some(s) => s.authentication.clone(),
        None => Authentication::NotAuthenticated,
    }
}

pub async fn get_user<H: AsRef<Headers>>(headers: H, store: &Store) -> Option<Arc<RwLock<User>>> {
    return get_authentication(headers, store).get_user();
}

#[cfg(test)]
mod tests {
    use crate::lib::id::ID;
    use tide::http::Url;

    #[test]
    fn test_get_session_id() {
        let url = Url::parse("http://example.org/").unwrap();
        let mut request = tide::http::Request::new(tide::http::Method::Post, url);

        request.insert_header("Authorization", "Bearer 3wB4St9NzSaC4r6ouj56eyRku22n");

        assert_eq!(
            super::get_session_id(request),
            "3wB4St9NzSaC4r6ouj56eyRku22n".parse::<ID>().ok()
        );
    }
}
*/
