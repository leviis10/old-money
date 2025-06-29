use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub enum ErrorCode {
    NotFound,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    code: ErrorCode,
    message: String,
}

impl ErrorResponse {
    pub fn new(code: ErrorCode, message: &str) -> ErrorResponse {
        ErrorResponse {
            code,
            message: String::from(message),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
