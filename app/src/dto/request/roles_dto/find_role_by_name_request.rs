use crate::enums::roles::Roles;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindRoleByNameRequest {
    pub role: Roles,
}
