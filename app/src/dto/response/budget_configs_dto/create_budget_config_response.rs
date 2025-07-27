use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBudgetConfigResponse {
    pub id: i32,

    pub name: String,

    pub duration: i32,

    pub limit: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub last_create: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}
