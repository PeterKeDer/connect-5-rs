use actix_web::http::StatusCode;
use serde::Serialize;
use crate::models::AppErrorType;

/// The error caused by an invalid operation to the state.
#[derive(Serialize)]
pub struct StateError {
    reason: &'static str,
}

impl StateError {
    pub fn new(reason: &'static str) -> StateError {
        StateError {
            reason,
        }
    }
}

impl AppErrorType for StateError {
    fn error_type(&self) -> &'static str {
        "state"
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
