use serde::{Deserialize, Serialize};

use crate::{
    app_state::{event::Event, log::Writer, RequestState},
    lib::id::ID,
};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: ID,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Output {
    Success,
    SessionNotFound,
}

pub async fn run<'a, T: Writer>(state: RequestState<T>, input: Input) -> Output {
    let store = state.get_store().await;

    if store.sessions.get(&input.session_id).is_none() {
        return Output::SessionNotFound;
    }
    drop(store);

    state
        .write(Event::SessionLogout {
            session_id: input.session_id,
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

//         let result = run(
//             state,
//             Input {
//                 session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//             },
//         )
//         .await;
//         assert_eq!(result, Output::SessionNotFound)
//     }

//     #[async_std::test]
//     async fn test_run_success() {
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
//             .await
//             .write(Event::SessionLogin {
//                 session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//                 user_id: ID::from_str("2bQFgyUNCCRUs8SitkgBG8L37KL1").unwrap(),
//             })
//             .await;

//         let result = run(
//             state.clone(),
//             Input {
//                 session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//             },
//         )
//         .await;

//         let store = state.get_store().await;

//         assert_eq!(result, Output::Success);
//         assert!(store
//             .sessions
//             .get(&ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap())
//             .unwrap()
//             .authentication
//             .get_user()
//             .is_none());
//     }
// }
