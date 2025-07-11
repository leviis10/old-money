use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterUserRequest {
    pub username: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(custom(function = "crate::utils::validation::validate_password"))]
    pub password: String,
}
