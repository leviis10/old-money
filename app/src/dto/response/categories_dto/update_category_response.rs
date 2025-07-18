use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryResponse {
    pub id: u32,

    pub name: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}
