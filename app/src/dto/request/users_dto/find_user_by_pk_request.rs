use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindUserByPkRequest {
    pub user_id: i32,
}
