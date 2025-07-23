use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateSelfRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,

    pub password: String,
}
