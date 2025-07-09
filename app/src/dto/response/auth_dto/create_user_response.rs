use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub id: i32,

    pub username: String,

    pub email: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}
