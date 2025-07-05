use crate::entities::roles::Model as RolesModel;
use crate::entities::users::Model as UsersModel;
use crate::enums::roles::Roles;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    roles: Vec<Roles>,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(user_model: UsersModel, roles_model: Vec<RolesModel>) -> Claims {
        let now = OffsetDateTime::now_utc();
        let iat = now.unix_timestamp();
        let exp = (now + Duration::days(30)).unix_timestamp();
        let sub = user_model.username.clone();
        let roles: Vec<Roles> = roles_model
            .iter()
            .map(|role_model| {
                return if role_model.name == "ADMIN" {
                    Roles::Admin
                } else {
                    Roles::Unknown
                };
            })
            .collect();
        Claims {
            sub,
            roles,
            iat: iat as usize,
            exp: exp as usize,
        }
    }
}

pub fn generate_token(claims: Claims) -> String {
    let secret = "secret";
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
