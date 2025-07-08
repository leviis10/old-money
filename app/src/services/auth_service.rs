use crate::dto::request::auth_dto::create_user_request::CreateUserRequest;
use crate::dto::request::auth_dto::login_user_request::LoginUserRequest;
use crate::entities::{roles, users};
use crate::enums::roles::Roles;
use crate::services::{refresh_tokens_service, users_service};
use crate::utils::jwt_utils;
use crate::utils::jwt_utils::{AccessTokenClaims, RefreshTokenClaims};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sea_orm::{ActiveValue, DatabaseConnection};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::dto::request::auth_dto::refresh_token_request::RefreshTokenRequest;

pub async fn register(
    db_connection: &DatabaseConnection,
    payload: &CreateUserRequest,
) -> users::Model {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = users::ActiveModel {
        username: ActiveValue::Set(String::from(&payload.username)),
        email: ActiveValue::Set(String::from(&payload.email)),
        password: ActiveValue::Set(hashed_password),
        ..Default::default()
    };

    users_service::create(db_connection, new_user, vec![Roles::Admin]).await
}

pub async fn login(
    db: &DatabaseConnection,
    payload: LoginUserRequest,
) -> Result<(String, String), String> {
    let (found_user, roles) = users_service::find_by_username(db, payload.username.as_str()).await;

    let parsed_hash = PasswordHash::new(&found_user.password).unwrap();
    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err(String::from("Invalid"));
    }

    let now = OffsetDateTime::now_utc();
    let (access_token, refresh_token) = generate_token(db, found_user.id, &roles, now).await;

    Ok((access_token, refresh_token))
}

pub async fn refresh(db: &DatabaseConnection, request: RefreshTokenRequest) -> (String, String) {
    let refresh_token_claims = RefreshTokenClaims::parse(&request.refresh_token);

    let refresh_token_expiration = OffsetDateTime::from_unix_timestamp(refresh_token_claims.exp as i64).unwrap();
    let refresh_token_model = refresh_tokens_service::find_by_pk_and_deleted_at_is_not_null_and_expires_at_less_than(db, refresh_token_claims.jti, refresh_token_expiration).await;

    match RefreshTokenClaims::compare_hash(request.refresh_token.as_bytes(), &refresh_token_model.hashed_token) {
        Ok(_) => {
            refresh_tokens_service::revoke_using_model(db, refresh_token_model).await;

            let (found_user, roles) = users_service::find_by_pk(db, refresh_token_claims.sub).await;
            let now = OffsetDateTime::now_utc();
            let (access_token, refresh_token) = generate_token(db, found_user.id, &roles, now).await;

            (access_token, refresh_token)
        },
        Err(_) => (String::from("INVALID"), String::from("INVALID"))
    }
}

async fn generate_token(
    db: &DatabaseConnection,
    user_id: i32,
    roles: &Vec<roles::Model>,
    now: OffsetDateTime,
) -> (String, String) {
    let access_token = generate_access_token(user_id, &roles, &now);
    let refresh_token = generate_refresh_token(&db, user_id, &now).await;

    (access_token, refresh_token)
}

fn generate_access_token(
    user_id: i32,
    roles: &Vec<roles::Model>,
    now: &OffsetDateTime,
) -> String {
    jwt_utils::generate_token(AccessTokenClaims::new(user_id, &roles, *now))
}

async fn generate_refresh_token(
    db: &DatabaseConnection,
    user_id: i32,
    now: &OffsetDateTime,
) -> String {
    let uuid = Uuid::now_v7();

    let (refresh_token_claims, expires_at) =
        RefreshTokenClaims::new(user_id, uuid, *now);
    let refresh_token = jwt_utils::generate_token(refresh_token_claims);
    let hashed_refresh_token = RefreshTokenClaims::hash(refresh_token.as_bytes());

    refresh_tokens_service::create(db, uuid, hashed_refresh_token, expires_at, user_id).await;

    refresh_token
}
