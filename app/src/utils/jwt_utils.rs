use crate::entities::roles;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

pub trait JwtToken {}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    sub: String,
    roles: Vec<String>,
    iat: usize,
    exp: usize,
}

impl JwtToken for AccessTokenClaims {}

impl AccessTokenClaims {
    pub fn new(
        username: &str,
        roles_model: &Vec<roles::Model>,
        from_time: OffsetDateTime,
    ) -> AccessTokenClaims {
        let iat = from_time.unix_timestamp();
        let exp = (from_time + Duration::minutes(5)).unix_timestamp() as usize;
        let roles: Vec<String> = roles_model
            .iter()
            .map(|role_model| String::from(&role_model.name))
            .collect();
        AccessTokenClaims {
            sub: String::from(username),
            roles,
            iat: iat as usize,
            exp,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    sub: String,
    jti: String,
    iat: usize,
    exp: usize,
}

impl JwtToken for RefreshTokenClaims {}

impl RefreshTokenClaims {
    pub fn new(username: &str, jti: &str, from_time: OffsetDateTime) -> RefreshTokenClaims {
        let iat = from_time.unix_timestamp();
        let exp = (from_time + Duration::days(30)).unix_timestamp() as usize;
        RefreshTokenClaims {
            sub: String::from(username),
            jti: String::from(jti),
            iat: iat as usize,
            exp,
        }
    }
}

pub fn generate_token<T: JwtToken + Serialize>(claims: T) -> String {
    let secret = std::env::var("JWT_SECRET").unwrap();
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
