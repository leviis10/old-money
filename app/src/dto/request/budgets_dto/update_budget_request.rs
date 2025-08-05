use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBudgetRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    // TODO: add limit validation
    pub limit: String,

    pub description: Option<String>,
}
