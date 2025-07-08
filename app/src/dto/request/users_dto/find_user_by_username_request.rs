use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindUserByUsernameRequest {
    pub username: String,
}
