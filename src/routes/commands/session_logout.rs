use crate::app_state::event::Event;
use crate::app_state::AppRequest;
use crate::lib::id::Id;
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
        .write(Event::SessionLogout {
            session_id: input.session_id,
        })
        .await;

    Output::Success
}

#[cfg(test)]
mod tests {
    use crate::lib::testutils::base_state;
    use std::str::FromStr;

    use super::run;
    use super::Input;
    use super::Output;

    use crate::lib::id::Id;

    #[async_std::test]
    async fn test_run_unknown_session() {
        let app_state = base_state().await;

        let result = run(
            app_state.into_request_state_current_time(),
            Input {
                session_id: Id::from_str("3hbzu4vGorn5PdG3HtiW4UQe784R").unwrap(),
            },
        )
        .await;
        assert_eq!(result, Output::SessionNotFound)
    }

    #[async_std::test]
    async fn test_run_success() {
        let app_state = base_state().await;

        let result = run(
            app_state.clone().into_request_state_current_time(),
            Input {
                session_id: Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap(),
            },
        )
        .await;

        let store = app_state.get_store().await;

        assert_eq!(result, Output::Success);
        assert!(store
            .sessions
            .get(&Id::from_str("3zCD548f6YU7163rZ84ZGamWkQM").unwrap())
            .unwrap()
            .authentication
            .get_user()
            .is_none());
    }
}
