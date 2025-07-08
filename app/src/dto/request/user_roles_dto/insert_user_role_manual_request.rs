use serde::Deserialize;

#[derive(Deserialize)]
pub struct InsertUserRoleManualRequest {
    pub user_id: i32,
    pub role_id: i32,
}
