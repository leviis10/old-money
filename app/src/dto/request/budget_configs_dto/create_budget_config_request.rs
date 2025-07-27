use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateBudgetConfigRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    #[validate(range(min = 1, message = "Duration must be greater than 0"))]
    pub duration: i32,

    pub limit: String,

    pub description: Option<String>,
}
