use serde::Serialize;
use time::Date;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBudgetResponse {
    pub name: String,

    pub start_date: Date,

    pub end_date: Date,

    pub current_amount: String,

    pub limit: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
