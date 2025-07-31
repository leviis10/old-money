use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBudgetConfigRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    pub limit: String,

    pub description: Option<String>,

    pub repetition_type: String,
}
