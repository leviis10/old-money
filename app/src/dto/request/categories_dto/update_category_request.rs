use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 3, message = "Length must be at least 3 characters long"))]
    pub name: String,
}
