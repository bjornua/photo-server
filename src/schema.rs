use crate::{
    app_state::{self, LockedAppState},
    timeline,
};
use juniper::FieldResult;
use juniper::RootNode;

use juniper::GraphQLObject;
use timeline::get_pictures;

impl juniper::Context for LockedAppState {}

#[derive(GraphQLObject)]
struct Photo {
    id: String,
    hash: String,
    width: f64,
    height: f64,
    date: chrono::DateTime<chrono::Utc>,
    size: f64,
    added: chrono::DateTime<chrono::Utc>,
}

#[derive(GraphQLObject)]
struct Session {
    id: String,
    user: Option<User>,
}

impl From<&app_state::Session> for Session {
    fn from(s: &app_state::Session) -> Self {
        return Self {
            id: s.token.encode(),
            user: s.user.as_ref().map(|u| u.into()),
        };
    }
}

#[derive(GraphQLObject)]
struct User {
    id: String,
}

impl From<&app_state::User> for User {
    fn from(u: &app_state::User) -> Self {
        return Self { id: u.id.clone() };
    }
}

#[derive(Debug)]
pub struct QueryRoot(pub LockedAppState);

#[juniper::object]
impl QueryRoot {
    fn photos() -> FieldResult<Vec<Photo>> {
        let paths_native = get_pictures();

        Ok(timeline::get_pictures()
            .into_iter()
            .filter_map(|p| p.to_str().map(|p| p.to_string()))
            .map(|p| Photo {
                id: p,
                hash: String::from("Test Hash"),
                width: 123.0,
                height: 123.0,
                date: chrono::MIN_DATETIME,
                size: 123.0,
                added: chrono::MIN_DATETIME,
            })
            .collect())
    }

    fn sessions(&self) -> Vec<Session> {
        let app_state = self.0 .0.read().unwrap();
        let test = app_state
            .sessions
            .values()
            .map(|session| session.into())
            .collect();

        test
    }
}

#[derive(Debug)]
pub struct MutationRoot(pub LockedAppState);

#[juniper::object]
impl MutationRoot {
    fn new_session(&self) -> Session {
        let mut app_state = self.0 .0.write().unwrap();
        let session = crate::app_state::Session::new();
        let session = app_state
            .sessions
            .entry(session.token.clone())
            .or_insert(session);
        return (&*session).into();
    }
}

pub type Schema<'a> = RootNode<'a, QueryRoot, MutationRoot>;
