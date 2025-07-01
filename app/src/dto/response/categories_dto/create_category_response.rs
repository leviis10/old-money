use derive_builder::Builder;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize, Builder)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryResponse {
    id: i32,

    name: String,

    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
}
