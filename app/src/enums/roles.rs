use serde::Deserialize;

#[derive(Deserialize)]
pub enum Roles {
    Admin,
}

impl Roles {
    pub fn get_name(&self) -> &'static str {
        match self {
            Roles::Admin => "ADMIN",
        }
    }
}
