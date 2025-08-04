use crate::AppState;
use crate::dto::request::auth_dto::login_user_request::LoginUserRequest;
use crate::dto::request::auth_dto::refresh_token_request::RefreshTokenRequest;
use crate::dto::request::auth_dto::register_user_request::RegisterUserRequest;
use crate::dto::response::auth_dto::create_user_response::CreateUserResponse;
use crate::dto::response::auth_dto::login_user_response::LoginUserResponse;
use crate::dto::response::auth_dto::refresh_token_response::RefreshTokenResponse;
use crate::dto::response::global::success_response::SuccessResponse;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::services::auth_service;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

#[utoipa::path(
    path = "/api/v1/auth/register",
    post,
    tag = "auth",
    operation_id = "auth_register",
    request_body(
        content = RegisterUserRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, body = SuccessResponse<CreateUserResponse>)
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RegisterUserRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateUserResponse>), AppError> {
    let user_model = auth_service::register(&state.db, &payload).await?;

    let response = SuccessResponse::new(
        "Success create new user",
        CreateUserResponse {
            id: user_model.id,
            username: user_model.username,
            email: user_model.email,
        },
    );

    Ok((StatusCode::CREATED, response))
}

#[utoipa::path(
    path = "/api/v1/auth/login",
    post,
    tag = "auth",
    operation_id = "auth_login",
    request_body(
        content = LoginUserRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 200, body = SuccessResponse<LoginUserResponse>)
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<LoginUserRequest>,
) -> Result<(StatusCode, SuccessResponse<LoginUserResponse>), AppError> {
    let (access_token, refresh_token) = auth_service::login(&state.db, payload).await?;

    let response = SuccessResponse::new(
        "Successfully logged in",
        LoginUserResponse {
            access_token,
            refresh_token,
        },
    );

    Ok((StatusCode::OK, response))
}

#[utoipa::path(
    path = "/api/v1/auth/refresh",
    post,
    tag = "auth",
    operation_id = "auth_refresh",
    request_body(
        content = RefreshTokenRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 200, body = SuccessResponse<RefreshTokenResponse>)
    )
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RefreshTokenRequest>,
) -> Result<(StatusCode, SuccessResponse<RefreshTokenResponse>), AppError> {
    let (access_token, refresh_token) = auth_service::refresh(&state.db, payload).await?;
    let response = SuccessResponse::new(
        "Successfully refresh token",
        RefreshTokenResponse {
            access_token,
            refresh_token,
        },
    );
    Ok((StatusCode::OK, response))
}
