use crate::AppState;
use crate::dto::request::auth_dto::{
    create_user_request::CreateUserRequest, login_user_request::LoginUserRequest,
};
use crate::dto::response::auth_dto::{
    create_user_response::{CreateUserResponse, CreateUserResponseBuilder},
    login_user_response::{LoginUserResponse, LoginUserResponseBuilder},
};
use crate::dto::response::global::success_response::SuccessResponse;
use crate::entities::{prelude::Users, users};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use time::OffsetDateTime;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, SuccessResponse<CreateUserResponse>) {
    // hash the password
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // store the user on database
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(payload.username.clone()),
        email: ActiveValue::Set(payload.email.clone()),
        password: ActiveValue::Set(hashed_password),
        ..Default::default()
    };
    let db_response = Users::insert(new_user).exec(&state.db).await.unwrap();

    // return success response
    let now = OffsetDateTime::now_utc();
    let response = SuccessResponse::new(
        "Success create new user",
        CreateUserResponseBuilder::default()
            .id(db_response.last_insert_id)
            .username(payload.username)
            .email(payload.email)
            .created_at(now)
            .updated_at(now)
            .build()
            .unwrap(),
    );

    (StatusCode::CREATED, response)
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUserRequest>,
) -> (StatusCode, SuccessResponse<LoginUserResponse>) {
    // check the username on the database
    let found_user = Users::find()
        .filter(users::Column::Username.eq(payload.username))
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    // compare the password on database and on the req body
    let parsed_hash = PasswordHash::new(&found_user.password).unwrap();
    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        let response = SuccessResponse::new(
            "wrong password",
            LoginUserResponseBuilder::default()
                .id(0)
                .username(found_user.username)
                .email(found_user.email)
                .created_at(found_user.created_at)
                .updated_at(found_user.updated_at)
                .build()
                .unwrap(),
        );
        return (StatusCode::BAD_REQUEST, response);
    }

    // response
    let response = SuccessResponse::new(
        "Successfully logged in",
        LoginUserResponseBuilder::default()
            .id(found_user.id)
            .username(found_user.username)
            .email(found_user.email)
            .created_at(found_user.created_at)
            .updated_at(found_user.updated_at)
            .build()
            .unwrap(),
    );
    (StatusCode::OK, response)
}
