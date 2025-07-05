use crate::AppState;
use crate::dto::request::auth_dto::{
    create_user_request::CreateUserRequest, login_user_request::LoginUserRequest,
};
use crate::dto::response::auth_dto::{
    create_user_response::{CreateUserResponse, CreateUserResponseBuilder},
    login_user_response::{LoginUserResponse, LoginUserResponseBuilder},
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
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, SuccessResponse<CreateUserResponse>) {
    let user_model = auth_service::register(&state.db, payload).await;

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
    match auth_service::login(&state.db, payload).await {
        Ok(token) => {
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
        Err(message) => {
            let response = SuccessResponse::new(
                &message,
                LoginUserResponseBuilder::default()
                    .access_token(&message)
                    .refresh_token(&message)
                    .build()
                    .unwrap(),
            );
            (StatusCode::BAD_REQUEST, response)
        }
    }
}
