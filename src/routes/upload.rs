use crate::{app_state::log::Writer, routes::commands};

use crate::app_state::AppState;

use serde::{Deserialize, Serialize};
use tide::{Request, Response, StatusCode};

pub async fn handle<T: Writer>(mut req: Request<AppState<T>>) -> tide::Result<impl Into<Response>> {
    

    let command_input: Input = match req.take_body().into_json().await {
        Ok(input) => input,
        Err(err) => {
            let err = err.downcast::<serde_json::Error>();
            return Ok(match err {
                Ok(serde_err) => {
                    let mut response = Response::new(StatusCode::UnprocessableEntity);
                    response.set_body(serde_err.to_string());
                    response
                }
                Err(err) => {
                    println!("Error: {}", err);
                    Response::new(StatusCode::UnprocessableEntity)
                }
            });
        }
    };

    let state = req.state();
    let state = state.clone().into_request_state_current_time();


    return match serde_json::to_value(result) {
        Ok(value) => Ok(Response::from(value)),
        Err(err) => {
            println!("Error serializing response: {}", err);
            Ok(Response::new(StatusCode::UnprocessableEntity))
        }
    };
}
