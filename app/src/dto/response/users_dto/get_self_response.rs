use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSelfResponse {
    pub username: String,

    pub email: String,

    pub created_at: OffsetDateTime,
}
