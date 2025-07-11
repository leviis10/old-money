use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub enum ErrorCode {
    NotFound,
    PasswordHashError,
    MissingEnvironmentVariable,
    ParsingError,
    DatabaseError,
    ValidationError
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
