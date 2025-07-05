use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Roles {
    Admin,
    Unknown,
}

impl Roles {
    pub fn get_name(&self) -> &'static str {
        match self {
            Roles::Admin => "ADMIN",
            Roles::Unknown => "UNKNOWN",
        }
    }
}
