use crate::entities::roles;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub trait JwtToken {}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    sub: i32,
    roles: Vec<String>,
    iat: usize,
    exp: usize,
}

impl JwtToken for AccessTokenClaims {}

impl AccessTokenClaims {
    pub fn new(
        user_id: i32,
        roles_model: &Vec<roles::Model>,
        from_time: OffsetDateTime,
    ) -> AccessTokenClaims {
        let access_token_expiration: i64 = std::env::var("ACCESS_TOKEN_EXPIRATION")
            .unwrap()
            .parse()
            .unwrap();
        let iat = from_time.unix_timestamp();
        let exp =
            (from_time + Duration::seconds(access_token_expiration)).unix_timestamp() as usize;
        let roles: Vec<String> = roles_model
            .iter()
            .map(|role_model| String::from(&role_model.name))
            .collect();
        AccessTokenClaims {
            sub: user_id,
            roles,
            iat: iat as usize,
            exp,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: i32,
    pub jti: Uuid,
    iat: usize,
    pub exp: usize,
}

impl JwtToken for RefreshTokenClaims {}

impl RefreshTokenClaims {
    pub fn new(
        user_id: i32,
        jti: Uuid,
        from_time: OffsetDateTime,
    ) -> (RefreshTokenClaims, OffsetDateTime) {
        let refresh_token_expiration: i64 = std::env::var("REFRESH_TOKEN_EXPIRATION")
            .unwrap()
            .parse()
            .unwrap();
        let iat = from_time.unix_timestamp();
        let expires_at = from_time + Duration::seconds(refresh_token_expiration);
        let expires_at_timestamp = expires_at.unix_timestamp() as usize;
        let refresh_token_claims = RefreshTokenClaims {
            sub: user_id,
            jti,
            iat: iat as usize,
            exp: expires_at_timestamp,
        };

        (refresh_token_claims, expires_at)
    }

    pub fn hash(refresh_token: &[u8]) -> String {
        hex::encode(Sha256::digest(refresh_token))
    }

    pub fn parse(refresh_token: &str) -> RefreshTokenClaims {
        let secret = std::env::var("JWT_SECRET").unwrap();
        jsonwebtoken::decode::<RefreshTokenClaims>(
            refresh_token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .unwrap()
        .claims
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
