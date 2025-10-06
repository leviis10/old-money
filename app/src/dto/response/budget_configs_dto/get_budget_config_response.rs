use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBudgetConfigResponse {
    pub id: i32,

    pub name: String,

    pub limit: String,

    pub description: Option<String>,

    pub repetition_type: String,
}
