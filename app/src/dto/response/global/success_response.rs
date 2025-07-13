use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    total_items: u64,
    page: u64,
    page_size: u64,
}

impl Meta {
    pub fn new(total_items: u64, page: u64, page_size: u64) -> Meta {
        Meta {
            total_items,
            page,
            page_size,
        }
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<T: Serialize> {
    message: String,

    data: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(message: &str, data: T) -> SuccessResponse<T> {
        SuccessResponse {
            message: String::from(message),
            data,
            meta: None,
        }
    }

    pub fn with_meta(mut self, meta: Meta) -> SuccessResponse<T> {
        self.meta = Some(meta);
        self
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
