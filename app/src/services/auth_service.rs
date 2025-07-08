use crate::dto::request::auth_dto::create_user_request::CreateUserRequest;
use crate::dto::request::auth_dto::login_user_request::LoginUserRequest;
use crate::dto::request::auth_dto::refresh_token_request::RefreshTokenRequest;
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
    let new_refresh_token_jti = Uuid::now_v7();
    let (access_token, refresh_token, expires_at) =
        generate_token(new_refresh_token_jti, found_user.id, &roles, now).await;
    refresh_tokens_service::create(
        db,
        new_refresh_token_jti,
        &refresh_token,
        expires_at,
        found_user.id,
    )
    .await;

    Ok((access_token, refresh_token))
}

pub async fn refresh(db: &DatabaseConnection, request: RefreshTokenRequest) -> (String, String) {
    let refresh_token_claims = RefreshTokenClaims::parse(&request.refresh_token);

    let hashed_token = RefreshTokenClaims::hash(request.refresh_token.as_bytes());
    let refresh_token_expiration =
        OffsetDateTime::from_unix_timestamp(refresh_token_claims.exp as i64).unwrap();
    let refresh_token_model = refresh_tokens_service::find_by_pk_and_hashed_token_and_user_id_and_expires_at_greater_than_and_deleted_at_is_null(db, refresh_token_claims.jti, &hashed_token, refresh_token_claims.sub, refresh_token_expiration).await;

    match refresh_token_model {
        Some(refresh_token_model) => {
            refresh_tokens_service::revoke_using_model(db, refresh_token_model).await;

            let (found_user, roles) = users_service::find_by_pk(db, refresh_token_claims.sub).await;
            let now = OffsetDateTime::now_utc();
            let new_refresh_token_jti = Uuid::now_v7();
            let (access_token, refresh_token, expires_at) =
                generate_token(new_refresh_token_jti, found_user.id, &roles, now).await;
            refresh_tokens_service::create(
                db,
                new_refresh_token_jti,
                &refresh_token,
                expires_at,
                found_user.id,
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
