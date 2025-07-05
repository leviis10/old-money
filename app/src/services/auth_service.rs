use crate::dto::request::auth_dto::create_user_request::CreateUserRequest;
use crate::dto::request::auth_dto::login_user_request::LoginUserRequest;
use crate::entities::users;
use crate::enums::roles::Roles;
use crate::services::users_service;
use crate::utils::jwt_utils;
use crate::utils::jwt_utils::{AccessTokenClaims, RefreshTokenClaims};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sea_orm::{ActiveValue, DatabaseConnection};
use time::OffsetDateTime;

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
    let access_token =
        jwt_utils::generate_token(AccessTokenClaims::new(&found_user.username, &roles, now));
    let refresh_token =
        jwt_utils::generate_token(RefreshTokenClaims::new(&found_user.username, "jti", now));
    Ok((access_token, refresh_token))
}
