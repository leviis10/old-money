use crate::entities::roles;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

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
    pub sub: String,
    pub jti: Uuid,
    iat: usize,
    pub exp: usize,
}

impl JwtToken for RefreshTokenClaims {}

impl RefreshTokenClaims {
    pub fn new(
        username: &str,
        jti: Uuid,
        from_time: OffsetDateTime,
    ) -> (RefreshTokenClaims, OffsetDateTime) {
        let iat = from_time.unix_timestamp();
        let expires_at = from_time + Duration::days(30);
        let expires_at_timestamp = expires_at.unix_timestamp() as usize;
        let refresh_token_claims = RefreshTokenClaims {
            sub: String::from(username),
            jti,
            iat: iat as usize,
            exp: expires_at_timestamp,
        };

        (refresh_token_claims, expires_at)
    }

    pub fn hash(refresh_token: &[u8]) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(refresh_token, &salt)
            .unwrap()
            .to_string()
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

    pub fn compare_hash(refresh_token: &[u8], hashed_refresh_token: &str) -> password_hash::errors::Result<()> {
        let parsed_hash = PasswordHash::new(hashed_refresh_token)?;
        Argon2::default().verify_password(refresh_token, &parsed_hash)
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
