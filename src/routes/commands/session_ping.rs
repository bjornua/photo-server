use crate::{
    app_state::{self, log::Writer, RequestState},
    lib::id::Id,
};
use app_state::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub session_id: Id,
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
        .write(Event::SessionPing {
            session_id: input.session_id,
        })
        .await;

    return Output::Success;
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use app_state::event::{DateEvent, Event};
//     use chrono::{TimeZone, Utc};

//     use super::{run, Input, Output};

//     use crate::{
//         app_state::{self, AppState},
//         lib::id::ID,
//     };

//     #[async_std::test]
//     async fn test_run_unknown_session() {
//         let state = AppState::new().into_request_state_current_time();
//         let session_id = ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap();
//         let result = run(state, Input { session_id }).await;
//         assert_eq!(result, Output::SessionNotFound)
//     }

//     #[async_std::test]
//     async fn test_run_success() {
//         let app_state = AppState::new();
//         let app_state = app_state
//             .write(DateEvent {
//                 date: Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 1, 444),
//                 kind: Event::SessionCreate {
//                     session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//                 },
//             })
//             .await;

//         let result = run(
//             app_state
//                 .clone()
//                 .into_request_state(Utc.ymd(1970, 1, 1).and_hms_milli(0, 10, 1, 123)),
//             Input {
//                 session_id: ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
//             },
//         )
//         .await;

//         let store = app_state.get_store().await;

//         assert_eq!(result, Output::Success);
//         assert_eq!(
//             store
//                 .sessions
//                 .get(&ID::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap())
//                 .unwrap()
//                 .last_ping,
//             Utc.ymd(1970, 1, 1).and_hms_milli(0, 10, 1, 123)
//         );
//     }
// }
