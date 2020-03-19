mod validation_error;
mod state_error;

pub use validation_error::*;
pub use state_error::*;

use std::fmt;
use std::sync;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::{Value, json};

pub trait AppErrorType {
    fn error_type(&self) -> &'static str {
        "unknown"
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug)]
pub struct AppError {
    value: Value,
    status: StatusCode,
}

impl AppError {
    fn internal() -> AppError {
        AppError {
            value: json!({
                "type": "internal_server_error",
            }),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = self.status.canonical_reason()
            .unwrap_or_else(|| self.status.as_str());

        write!(f, "{}: {}", status_str, self.value)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status)
            .json(&self.value)
    }
}

impl<T> From<T> for AppError
where
    T: AppErrorType + Serialize
{
    fn from(error: T) -> AppError {
        AppError {
            value: json!({
                "type": error.error_type(),
                "error": error,
            }),
            status: error.status_code(),
        }
    }
}

impl<Guard> From<sync::PoisonError<Guard>> for AppError {
    fn from(_: sync::PoisonError<Guard>) -> AppError {
        AppError::internal()
    }
}
