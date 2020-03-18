use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::{Value, json};

#[derive(Debug)]
pub struct Error {
    value: Value,
    status: StatusCode,
}

impl Error {
    fn new(status: StatusCode, msg: &str) -> Error {
        Error {
            value: json!({
                "error": msg,
            }),
            status,
        }
    }

    pub fn bad_request(msg: &str) -> Error {
        println!("{}", msg);
        Error::new(StatusCode::BAD_REQUEST, msg)
    }

    pub fn internal() -> Error {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = self.status.canonical_reason()
            .unwrap_or_else(|| self.status.as_str());

        write!(f, "{}: {}", status_str, self.value)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status)
            .json(&self.value)
    }
}

// TODO: implement From trait for Error
