use crate::app_state;
use crate::app_state::AppRequest;
use crate::lib::id::Id;
use app_state::event::Event;
use serde::Deserialize;
use serde::Serialize;

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

pub async fn run(state: AppRequest, input: Input) -> Output {
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

    Output::Success
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use app_state::event::DateEvent;
    use app_state::event::Event;
    use chrono::TimeZone;
    use chrono::Utc;

    use super::run;
    use super::Input;
    use super::Output;

    use crate::app_state;
    use crate::lib::id::Id;
    use crate::lib::testutils;

    #[async_std::test]
    async fn test_run_unknown_session() {
        let state = testutils::test_state()
            .await
            .into_request_state_current_time();
        let session_id = Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap();
        let result = run(state, Input { session_id }).await;
        assert_eq!(result, Output::SessionNotFound)
    }

    #[async_std::test]
    async fn test_run_success() {
        let app_state = testutils::test_state().await;
        let app_state = app_state
            .write(DateEvent {
                date: Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 1, 444),
                kind: Event::SessionCreate {
                    session_id: Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
                },
            })
            .await;

        let result = run(
            app_state
                .clone()
                .into_app_request(Utc.ymd(1970, 1, 1).and_hms_milli(0, 10, 1, 123)),
            Input {
                session_id: Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
            },
        )
        .await;

        let store = app_state.get_store().await;

        assert_eq!(result, Output::Success);
        assert_eq!(
            store
                .sessions
                .get(&Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap())
                .unwrap()
                .last_ping,
            Utc.ymd(1970, 1, 1).and_hms_milli(0, 10, 1, 123)
        );
    }
}
