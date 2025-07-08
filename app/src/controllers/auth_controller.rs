use crate::AppState;
use crate::dto::request::auth_dto::login_user_request::LoginUserRequest;
use crate::dto::request::auth_dto::refresh_token_request::RefreshTokenRequest;
use crate::dto::request::auth_dto::register_user_request::RegisterUserRequest;
use crate::dto::response::auth_dto::create_user_response::{
    CreateUserResponse, CreateUserResponseBuilder,
};
use crate::dto::response::auth_dto::login_user_response::{
    LoginUserResponse, LoginUserResponseBuilder,
};
use crate::dto::response::auth_dto::refresh_token_response::{
    RefreshTokenResponse, RefreshTokenResponseBuilder,
};
use crate::dto::response::global::success_response::SuccessResponse;
use crate::services::auth_service;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use time::OffsetDateTime;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserRequest>,
) -> (StatusCode, SuccessResponse<CreateUserResponse>) {
    let user_model = auth_service::register(&state.db, &payload).await;

    let now = OffsetDateTime::now_utc();
    let response = SuccessResponse::new(
        "Success create new user",
        CreateUserResponseBuilder::default()
            .id(user_model.id)
            .username(user_model.username)
            .email(user_model.email)
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
    let (access_token, refresh_token) = auth_service::login(&state.db, payload).await;

    let response = SuccessResponse::new(
        "Successfully logged in",
        LoginUserResponseBuilder::default()
            .access_token(access_token)
            .refresh_token(refresh_token)
            .build()
            .unwrap(),
    );
    (StatusCode::OK, response)
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> (StatusCode, SuccessResponse<RefreshTokenResponse>) {
    let (access_token, refresh_token) = auth_service::refresh(&state.db, payload).await;
    let response = RefreshTokenResponseBuilder::default()
        .access_token(access_token)
        .refresh_token(refresh_token)
        .build()
        .unwrap();
    (
        StatusCode::OK,
        SuccessResponse::new("Successfully refresh token", response),
    )
}
