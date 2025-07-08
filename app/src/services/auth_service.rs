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
use crate::services::{refresh_tokens_service, users_service};
use crate::utils::jwt_utils;
use crate::utils::jwt_utils::{AccessTokenClaims, RefreshTokenClaims};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sea_orm::DatabaseConnection;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn register(db: &DatabaseConnection, request: &RegisterUserRequest) -> users::Model {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    users_service::create(
        db,
        CreateUserRequest {
            username: String::from(&request.username),
            email: String::from(&request.email),
            hashed_password,
            roles: vec![Roles::Admin],
        },
    )
    .await
}

pub async fn login(db: &DatabaseConnection, request: LoginUserRequest) -> (String, String) {
    let (found_user, roles) = users_service::find_by_username(
        db,
        FindUserByUsernameRequest {
            username: request.username,
        },
    )
    .await;

    let parsed_hash = PasswordHash::new(&found_user.password).unwrap();
    match Argon2::default().verify_password(request.password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            let now = OffsetDateTime::now_utc();
            let new_refresh_token_jti = Uuid::now_v7();
            let (access_token, refresh_token, expires_at) =
                generate_token(new_refresh_token_jti, found_user.id, &roles, now).await;
            refresh_tokens_service::create(
                db,
                CreateRefreshTokenRequest {
                    jti: new_refresh_token_jti,
                    refresh_token: String::from(&refresh_token),
                    expires_at,
                    user_id: found_user.id,
                },
            )
            .await;
            (access_token, refresh_token)
        }
        Err(_) => (String::from("Invalid"), String::from("Invalid")),
    }
}

pub async fn refresh(db: &DatabaseConnection, request: RefreshTokenRequest) -> (String, String) {
    let refresh_token_claims = RefreshTokenClaims::parse(&request.refresh_token);

    let hashed_token = RefreshTokenClaims::hash(request.refresh_token.as_bytes());
    let refresh_token_expiration =
        OffsetDateTime::from_unix_timestamp(refresh_token_claims.exp as i64).unwrap();
    let found_refresh_token = refresh_tokens_service::find_by_pk(
        db,
        FindRefreshTokenByPkRequest {
            jti: refresh_token_claims.jti,
            hashed_token,
            user_id: refresh_token_claims.sub,
            expires_at: refresh_token_expiration,
        },
    )
    .await;

    match found_refresh_token {
        Some(refresh_token_model) => {
            refresh_tokens_service::revoke_by_jti(
                db,
                RevokeRefreshTokenByJtiRequest {
                    jti: refresh_token_model.jti,
                },
            )
            .await;

            let (found_user, roles) = users_service::find_by_pk(
                db,
                FindUserByPkRequest {
                    user_id: refresh_token_claims.sub,
                },
            )
            .await;

            let now = OffsetDateTime::now_utc();
            let new_refresh_token_jti = Uuid::now_v7();
            let (access_token, refresh_token, expires_at) =
                generate_token(new_refresh_token_jti, found_user.id, &roles, now).await;

            refresh_tokens_service::create(
                db,
                CreateRefreshTokenRequest {
                    jti: new_refresh_token_jti,
                    refresh_token: String::from(&refresh_token),
                    expires_at,
                    user_id: found_user.id,
                },
            )
            .await;

            (access_token, refresh_token)
        }
        None => (String::from("INVALID"), String::from("INVALID")),
    }
}

async fn generate_token(
    jti: Uuid,
    user_id: i32,
    roles: &Vec<roles::Model>,
    now: OffsetDateTime,
) -> (String, String, OffsetDateTime) {
    let access_token = generate_access_token(user_id, &roles, &now);
    let (refresh_token, expires_at) = generate_refresh_token(jti, user_id, &now).await;

    (access_token, refresh_token, expires_at)
}

fn generate_access_token(user_id: i32, roles: &Vec<roles::Model>, now: &OffsetDateTime) -> String {
    jwt_utils::generate_token(AccessTokenClaims::new(user_id, &roles, *now))
}

async fn generate_refresh_token(
    jti: Uuid,
    user_id: i32,
    now: &OffsetDateTime,
) -> (String, OffsetDateTime) {
    let (refresh_token_claims, expires_at) = RefreshTokenClaims::new(user_id, jti, *now);
    let refresh_token = jwt_utils::generate_token(refresh_token_claims);

    (refresh_token, expires_at)
}
