use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

#[derive(Serialize, Debug)]
pub struct JsonError {
    pub msg: &'static str,
    pub status: u16,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg, });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .json(err_json)
    }
}
