use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}
