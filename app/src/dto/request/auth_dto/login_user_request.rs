use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}
