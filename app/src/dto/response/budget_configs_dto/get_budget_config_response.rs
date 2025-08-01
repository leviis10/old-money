use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBudgetConfigResponse {
    pub id: i32,

    pub name: String,

    pub limit: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub repetition_type: String,
}
