use crate::dto::response::global::error_response::{ErrorCode, ErrorResponse};
use axum::http::{StatusCode, Uri};

pub async fn not_found(uri: Uri) -> (StatusCode, ErrorResponse) {
    let response = ErrorResponse {
        code: ErrorCode::NotFound,
        message: format!("{} not found", uri.path()),
    };
    (StatusCode::NOT_FOUND, response)
}
