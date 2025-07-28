use crate::AppState;
use crate::dto::request::users_dto::delete_self_request::DeleteSelfRequest;
use crate::dto::request::users_dto::update_self_request::UpdateSelfRequest;
use crate::dto::response::global::success_response::SuccessResponse;
use crate::dto::response::users_dto::get_self_response::GetSelfResponse;
use crate::dto::response::users_dto::update_self_response::UpdateSelfResponse;
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::users_service;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

#[utoipa::path(
    path = "/api/v1/users/self",
    get,
    tag = "users",
    operation_id = "users_get_self",
    responses(
        (status = 200, body = SuccessResponse<GetSelfResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_self(
    User(found_user, roles): User,
) -> Result<(StatusCode, SuccessResponse<GetSelfResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Success get self data",
            GetSelfResponse {
                username: found_user.username,
                email: found_user.email,
                created_at: found_user.created_at,
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/users/self",
    put,
    tag = "users",
    operation_id = "users_update_self",
    request_body(
        content = UpdateSelfRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 200, body = SuccessResponse<UpdateSelfResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_self(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<UpdateSelfRequest>,
) -> Result<(StatusCode, SuccessResponse<UpdateSelfResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let updated_found_user =
        users_service::update_using_model(&state.db, found_user, &request).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully self update",
            UpdateSelfResponse {
                username: updated_found_user.username,
                email: updated_found_user.email,
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/users/self",
    delete,
    tag = "users",
    operation_id = "users_delete_self",
    request_body(
        content = DeleteSelfRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 204)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_self(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<DeleteSelfRequest>,
) -> Result<StatusCode, AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    users_service::delete_using_model(&state.db, found_user, &request).await?;
    Ok(StatusCode::NO_CONTENT)
}
