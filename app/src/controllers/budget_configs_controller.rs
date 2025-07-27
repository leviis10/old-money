use crate::AppState;
use crate::constants::time_constants::DATE_FORMATTER;
use crate::dto::request::budget_configs_dto::create_budget_config_request::CreateBudgetConfigRequest;
use crate::dto::request::budget_configs_dto::update_budget_config_request::UpdateBudgetConfigRequest;
use crate::dto::response::budget_configs_dto::create_budget_config_response::CreateBudgetConfigResponse;
use crate::dto::response::budget_configs_dto::get_budget_config_response::GetBudgetConfigResponse;
use crate::dto::response::budget_configs_dto::update_budget_config_response::UpdateBudgetConfigResponse;
use crate::dto::response::global::success_response::SuccessResponse;
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::budget_configs_service;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

#[utoipa::path(
    tag = "budget-configs",
    post,
    path = "/api/v1/budget-configs",
    request_body(
        content = CreateBudgetConfigRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, body = SuccessResponse<CreateBudgetConfigResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<CreateBudgetConfigRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateBudgetConfigResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let new_budget_config = budget_configs_service::create(&state.db, &found_user, request).await?;

    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(
            "Successfully create new Budget Config",
            CreateBudgetConfigResponse {
                id: new_budget_config.id,
                name: new_budget_config.name,
                duration: new_budget_config.duration,
                limit: new_budget_config.limit.to_string(),
                description: new_budget_config.description,
                last_create: new_budget_config.last_create.format(&DATE_FORMATTER)?,
                created_at: new_budget_config.created_at,
                updated_at: new_budget_config.updated_at,
            },
        ),
    ))
}

#[utoipa::path(
    tag = "budget-configs",
    get,
    path = "/api/v1/budget-configs",
    responses(
        (status = 200, body = SuccessResponse<Vec<GetBudgetConfigResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn find_all(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
) -> Result<(StatusCode, SuccessResponse<Vec<GetBudgetConfigResponse>>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let found_budget_configs = budget_configs_service::find_all(&state.db, &found_user).await?;
    let response: Vec<GetBudgetConfigResponse> = found_budget_configs
        .into_iter()
        .map(|budget_config| GetBudgetConfigResponse {
            id: budget_config.id,
            name: budget_config.name,
            duration: budget_config.duration,
            limit: budget_config.limit.to_string(),
            description: budget_config.description,
        })
        .collect();

    Ok((
        StatusCode::OK,
        SuccessResponse::new("Successfully fetch budget configs", response),
    ))
}

#[utoipa::path(
    tag = "budget-configs",
    get,
    path = "/api/v1/budget-configs/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = SuccessResponse<GetBudgetConfigResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    User(found_user, roles): User,
) -> Result<(StatusCode, SuccessResponse<GetBudgetConfigResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let found_budget_config = budget_configs_service::get_by_id(&state.db, &found_user, id).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully get a budget config",
            GetBudgetConfigResponse {
                id: found_budget_config.id,
                name: found_budget_config.name,
                duration: found_budget_config.duration,
                limit: found_budget_config.limit.to_string(),
                description: found_budget_config.description,
            },
        ),
    ))
}

#[utoipa::path(
    tag = "budget-configs",
    put,
    path = "/api/v1/budget-configs/{id}",
    params(
        ("id" = i32, Path)
    ),
    request_body(
        content = UpdateBudgetConfigRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 200, body = SuccessResponse<UpdateBudgetConfigResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<UpdateBudgetConfigRequest>,
) -> Result<(StatusCode, SuccessResponse<UpdateBudgetConfigResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let updated_budget_config =
        budget_configs_service::update_by_id(&state.db, &found_user, id, request).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully update budget config",
            UpdateBudgetConfigResponse {
                id: updated_budget_config.id,
                name: updated_budget_config.name,
                duration: updated_budget_config.duration,
                limit: updated_budget_config.limit.to_string(),
                description: updated_budget_config.description,
            },
        ),
    ))
}

#[utoipa::path(
    tag = "budget-configs",
    delete,
    path = "/api/v1/budget-configs/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 204)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    User(found_user, roles): User,
) -> Result<StatusCode, AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    budget_configs_service::delete_by_id(&state.db, &found_user, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
