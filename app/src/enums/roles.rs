use crate::entities::roles;
use crate::errors::AppError;
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum Roles {
    Admin,
    User,
}

impl Roles {
    pub fn get_name(&self) -> &'static str {
        match self {
            Roles::Admin => "ADMIN",
            Roles::User => "USER",
        }
    }

    fn from_string(role_name: &str) -> Result<Roles, AppError> {
        match role_name {
            "ADMIN" => Ok(Roles::Admin),
            "USER" => Ok(Roles::User),
            _ => Err(AppError::ParseRoleError),
        }
    }

    pub fn from_models(roles: Vec<roles::Model>) -> Result<Vec<Roles>, AppError> {
        roles
            .iter()
            .map(|role| Roles::from_string(&role.name))
            .collect()
    }
}
