use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}
