use crate::dto::response::global::error_response::{ErrorCode, ErrorResponse};
use axum::http::{StatusCode, Uri};

pub async fn not_found(uri: Uri) -> (StatusCode, ErrorResponse) {
    let response = ErrorResponse::new(
        ErrorCode::NotFound,
        format!("{} not found", uri.path()).as_str(),
    );
    (StatusCode::NOT_FOUND, response)
}
