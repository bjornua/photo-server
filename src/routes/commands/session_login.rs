use crate::{
    app_state::{event::Event, log::Writer, RequestState},
    lib::id::Id,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: Id,
    pub handle: String,
    pub password: String,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    AuthenticationFailed,
    SessionNotFound,
}

pub async fn run<'a, T: Writer>(state: RequestState<T>, input: Input) -> Output {
    let store = state.get_store().await;

    if store.sessions.get(&input.session_id).is_none() {
        return Output::SessionNotFound;
    }

    let user_ref = match store.users.get_by_handle(&input.handle) {
        Some(user) => user,
        None => return Output::AuthenticationFailed,
    };
    let user = user_ref.read().await;

    if user.password != input.password {
        return Output::AuthenticationFailed;
    }

    drop(store);

    state
        .write(Event::SessionLogin {
            session_id: input.session_id,
            user_id: user.id.clone(),
        })
        .await;

    return Output::Success;
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use app_state::{event::Event, AppState};

//     use super::{run, Input, Output};

//     use crate::{
//         app_state::{self},
//         lib::id::ID,
//     };

//     #[async_std::test]
//     async fn test_run_unknown_session() {
//         let state = AppState::new().into_request_state_current_time();
//         let session_id = ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap();
//         let result = run(
//             state,
//             Input {
//                 session_id,
//                 handle: "".to_string(),
//                 password: "".to_string(),
//             },
//         )
//         .await;
//         assert_eq!(result, Output::SessionNotFound)
//     }

//     #[async_std::test]
//     async fn test_run_bad_user() {
//         let state = AppState::new().into_request_state_current_time();
//         let session_id = ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap();
//         let state = state
//             .write(Event::SessionCreate {
//                 session_id: session_id.clone(),
//             })
//             .await;
//         let result = run(
//             state,
//             Input {
//                 session_id,
//                 handle: "".to_string(),
//                 password: "".to_string(),
//             },
//         )
//         .await;
//         assert_eq!(result, Output::AuthenticationFailed)
//     }

//     #[async_std::test]
//     async fn test_run_bad_password() {
//         let state = AppState::new().into_request_state_current_time();
//         let state = state
//             .write(Event::SessionCreate {
//                 session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//             })
//             .await
//             .write(Event::UserCreate {
//                 user_id: ID::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
//                 handle: "heidi".to_string(),
//                 name: "Heidi".to_string(),
//                 password: "eeQuee9t".to_string(),
//             })
//             .await;

//         let result = run(
//             state,
//             Input {
//                 session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//                 handle: "heidi".to_string(),
//                 password: "eeQuee9t".to_string(),
//             },
//         )
//         .await;
//         assert_eq!(result, Output::Success)
//     }

//     #[async_std::test]
//     async fn test_run_good_auth() {
//         let state = AppState::new().into_request_state_current_time();
//         let session_id = ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap();
//         let state = state
//             .write(Event::SessionCreate {
//                 session_id: session_id.clone(),
//             })
//             .await
//             .write(Event::UserCreate {
//                 user_id: ID::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
//                 handle: "heidi".to_string(),
//                 name: "Heidi".to_string(),
//                 password: "eeQuee9t".to_string(),
//             })
//             .await;

//         let result = run(
//             state.clone(),
//             Input {
//                 session_id,
//                 handle: "heidi".to_string(),
//                 password: "eeQuee9t".to_string(),
//             },
//         )
//         .await;

//         let store = state.get_store().await;

//         assert_eq!(result, Output::Success);
//         assert_eq!(
//             store
//                 .sessions
//                 .get(&ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap())
//                 .unwrap()
//                 .authentication
//                 .get_user()
//                 .unwrap()
//                 .read()
//                 .await
//                 .id,
//             ID::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap()
//         );
//     }
// }