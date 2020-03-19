use actix_web::http::StatusCode;
use serde::Serialize;
use crate::models::AppErrorType;

/// The error caused by an invalid input from user.
#[derive(Serialize)]
pub struct ValidationError {
    field: &'static str,
    reason: Option<&'static str>,
}

impl ValidationError {
    pub fn new(field: &'static str, reason: Option<&'static str>) -> ValidationError {
        ValidationError {
            field,
            reason,
        }
    }
}

impl AppErrorType for ValidationError {
    fn error_type(&self) -> &'static str {
        "validation"
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
