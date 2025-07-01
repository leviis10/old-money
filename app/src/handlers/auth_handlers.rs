use crate::AppState;
use crate::dto::request::auth_dto::{
    create_user_request::CreateUserRequest, login_user_request::LoginUserRequest,
};
use crate::dto::response::auth_dto::{
    create_user_response::{CreateUserResponse, CreateUserResponseBuilder},
    login_user_response::{LoginUserResponse, LoginUserResponseBuilder},
};
use crate::dto::response::global::success_response::SuccessResponse;
use crate::entities::prelude::Roles;
use crate::entities::{prelude::Users, roles, user_roles, users};
use crate::utils::jwt_utils;
use crate::utils::jwt_utils::Claims;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait,
};
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
    // start database transaction
    let txn = state.db.clone().begin().await.unwrap();

    // create new user
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(payload.username.clone()),
        email: ActiveValue::Set(payload.email.clone()),
        password: ActiveValue::Set(hashed_password),
        ..Default::default()
    };
    let user_model = new_user.insert(&txn).await.unwrap();

    // find 'ADMIN' role
    let admin_role_model = Roles::find()
        .filter(roles::Column::Name.eq("ADMIN"))
        .one(&txn)
        .await
        .unwrap()
        .unwrap();

    // insert the role on `UserRoles` table
    let new_user_role = user_roles::ActiveModel {
        user_id: ActiveValue::Set(user_model.id),
        role_id: ActiveValue::Set(admin_role_model.id),
    };
    new_user_role.insert(&txn).await.unwrap();

    // commit changes
    txn.commit().await.unwrap();

    // return success response
    let now = OffsetDateTime::now_utc();
    let response = SuccessResponse::new(
        "Success create new user",
        CreateUserResponseBuilder::default()
            .id(user_model.id)
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
    let found_users = Users::find()
        .filter(users::Column::Username.eq(payload.username))
        .find_with_related(Roles)
        .all(&state.db)
        .await
        .unwrap();
    let (found_user, roles) = &found_users[0];

    // compare the password on database and on the req body
    let parsed_hash = PasswordHash::new(&found_user.password).unwrap();
    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        let response = SuccessResponse::new(
            "wrong password",
            LoginUserResponseBuilder::default()
                .access_token(String::from("INVALID"))
                .refresh_token(String::from("INVALID"))
                .build()
                .unwrap(),
        );
        return (StatusCode::BAD_REQUEST, response);
    }

    // Generate JWT
    let claims = Claims::new(&found_user, roles);
    let token = jwt_utils::generate_token(claims);

    // response
    let response = SuccessResponse::new(
        "Successfully logged in",
        LoginUserResponseBuilder::default()
            .access_token(token.clone())
            .refresh_token(token)
            .build()
            .unwrap(),
    );
    (StatusCode::OK, response)
}
