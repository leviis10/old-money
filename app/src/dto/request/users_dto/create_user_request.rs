use crate::enums::roles::Roles;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub roles: Vec<Roles>,
}
