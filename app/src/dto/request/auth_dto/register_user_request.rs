use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterUserRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(custom(function = "crate::utils::validation::validate_password"))]
    pub password: String,
}
