use crate::entities::roles;
use crate::errors::AppError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub trait JwtToken {}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: i32,
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
    ) -> Result<AccessTokenClaims, AppError> {
        let access_token_expiration: i64 = std::env::var("ACCESS_TOKEN_EXPIRATION")?.parse()?;
        let iat = from_time.unix_timestamp();
        let exp =
            (from_time + Duration::seconds(access_token_expiration)).unix_timestamp() as usize;
        let roles: Vec<String> = roles_model
            .iter()
            .map(|role_model| String::from(&role_model.name))
            .collect();

        let access_token = AccessTokenClaims {
            sub: user_id,
            roles,
            iat: iat as usize,
            exp,
        };
        Ok(access_token)
    }

    pub fn parse(access_token: &str) -> Result<AccessTokenClaims, AppError> {
        let secret = std::env::var("JWT_SECRET")?;
        let access_token_claim: AccessTokenClaims = jsonwebtoken::decode(
            access_token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?
        .claims;
        Ok(access_token_claim)
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
    ) -> Result<(RefreshTokenClaims, OffsetDateTime), AppError> {
        let refresh_token_expiration: i64 = std::env::var("REFRESH_TOKEN_EXPIRATION")?.parse()?;
        let iat = from_time.unix_timestamp();
        let expires_at = from_time + Duration::seconds(refresh_token_expiration);
        let expires_at_timestamp = expires_at.unix_timestamp() as usize;
        let refresh_token_claims = RefreshTokenClaims {
            sub: user_id,
            jti,
            iat: iat as usize,
            exp: expires_at_timestamp,
        };

        Ok((refresh_token_claims, expires_at))
    }

    pub fn hash(refresh_token: &[u8]) -> String {
        hex::encode(Sha256::digest(refresh_token))
    }

    pub fn parse(refresh_token: &str) -> Result<RefreshTokenClaims, AppError> {
        let secret = std::env::var("JWT_SECRET")?;

        let refresh_token_claim: RefreshTokenClaims = jsonwebtoken::decode(
            refresh_token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?
        .claims;

        Ok(refresh_token_claim)
    }
}

pub fn generate_token<T: JwtToken + Serialize>(claims: T) -> Result<String, AppError> {
    let secret = std::env::var("JWT_SECRET")?;

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}
