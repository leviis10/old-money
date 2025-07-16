use crate::dto::request::auth_dto::login_user_request::LoginUserRequest;
use crate::dto::request::auth_dto::refresh_token_request::RefreshTokenRequest;
use crate::dto::request::auth_dto::register_user_request::RegisterUserRequest;
use crate::dto::request::refresh_tokens_dto::create_refresh_token_request::CreateRefreshTokenRequest;
use crate::dto::request::refresh_tokens_dto::find_refresh_token_by_pk_request::FindRefreshTokenByPkRequest;
use crate::dto::request::refresh_tokens_dto::revoke_refresh_token_by_jti_request::RevokeRefreshTokenByJtiRequest;
use crate::dto::request::users_dto::create_user_request::CreateUserRequest;
use crate::dto::request::users_dto::find_user_by_pk_request::FindUserByPkRequest;
use crate::dto::request::users_dto::find_user_by_username_request::FindUserByUsernameRequest;
use crate::entities::{roles, users};
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::services::{refresh_tokens_service, users_service};
use crate::utils::jwt_utils;
use crate::utils::jwt_utils::{AccessTokenClaims, RefreshTokenClaims};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sea_orm::DatabaseConnection;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn register(
    db: &DatabaseConnection,
    request: &RegisterUserRequest,
) -> Result<users::Model, AppError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(request.password.as_bytes(), &salt)?
        .to_string();

    let result = users_service::create(
        db,
        CreateUserRequest {
            username: String::from(&request.username),
            email: String::from(&request.email),
            hashed_password,
            roles: vec![Roles::User],
        },
    )
    .await?;

    Ok(result)
}

pub async fn login(
    db: &DatabaseConnection,
    request: LoginUserRequest,
) -> Result<(String, String), AppError> {
    let (found_user, roles) = users_service::find_by_username(
        db,
        FindUserByUsernameRequest {
            username: request.username,
        },
    )
    .await?;

    let parsed_hash = PasswordHash::new(&found_user.password)?;
    Argon2::default().verify_password(request.password.as_bytes(), &parsed_hash)?;

    let now = OffsetDateTime::now_utc();
    let new_refresh_token_jti = Uuid::now_v7();
    let (access_token, refresh_token, expires_at) =
        generate_token(new_refresh_token_jti, found_user.id, &roles, now).await?;
    refresh_tokens_service::create(
        db,
        CreateRefreshTokenRequest {
            jti: new_refresh_token_jti,
            refresh_token: String::from(&refresh_token),
            expires_at,
            user_id: found_user.id,
        },
    )
    .await?;
    Ok((access_token, refresh_token))
}

pub async fn refresh(
    db: &DatabaseConnection,
    request: RefreshTokenRequest,
) -> Result<(String, String), AppError> {
    let refresh_token_claims = RefreshTokenClaims::parse(&request.refresh_token)?;

    let hashed_token = RefreshTokenClaims::hash(request.refresh_token.as_bytes());
    let refresh_token_expiration =
        OffsetDateTime::from_unix_timestamp(refresh_token_claims.exp as i64)?;
    let found_refresh_token = refresh_tokens_service::find_by_pk(
        db,
        FindRefreshTokenByPkRequest {
            jti: refresh_token_claims.jti,
            hashed_token,
            user_id: refresh_token_claims.sub,
            expires_at: refresh_token_expiration,
        },
    )
    .await?;

    refresh_tokens_service::revoke_by_jti(
        db,
        RevokeRefreshTokenByJtiRequest {
            jti: found_refresh_token.jti,
        },
    )
    .await?;

    let (found_user, roles) = users_service::find_by_pk(
        db,
        FindUserByPkRequest {
            user_id: refresh_token_claims.sub,
        },
    )
    .await?;

    let now = OffsetDateTime::now_utc();
    let new_refresh_token_jti = Uuid::now_v7();
    let (access_token, refresh_token, expires_at) =
        generate_token(new_refresh_token_jti, found_user.id, &roles, now).await?;

    refresh_tokens_service::create(
        db,
        CreateRefreshTokenRequest {
            jti: new_refresh_token_jti,
            refresh_token: String::from(&refresh_token),
            expires_at,
            user_id: found_user.id,
        },
    )
    .await?;

    Ok((access_token, refresh_token))
}

async fn generate_token(
    jti: Uuid,
    user_id: i32,
    roles: &[roles::Model],
    now: OffsetDateTime,
) -> Result<(String, String, OffsetDateTime), AppError> {
    let access_token = generate_access_token(user_id, roles, &now)?;
    let (refresh_token, expires_at) = generate_refresh_token(jti, user_id, &now).await?;

    Ok((access_token, refresh_token, expires_at))
}

fn generate_access_token(
    user_id: i32,
    roles: &[roles::Model],
    now: &OffsetDateTime,
) -> Result<String, AppError> {
    let access_token_claim = AccessTokenClaims::new(user_id, roles, *now)?;
    let access_token = jwt_utils::generate_token(access_token_claim)?;
    Ok(access_token)
}

async fn generate_refresh_token(
    jti: Uuid,
    user_id: i32,
    now: &OffsetDateTime,
) -> Result<(String, OffsetDateTime), AppError> {
    let (refresh_token_claims, expires_at) = RefreshTokenClaims::new(user_id, jti, *now)?;
    let refresh_token = jwt_utils::generate_token(refresh_token_claims)?;

    Ok((refresh_token, expires_at))
}
