use crate::AppState;
use crate::dto::request::budget_configs_dto::update_budget_config_request::UpdateBudgetConfigRequest;
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
use sea_orm::ActiveEnum;
use std::sync::Arc;

#[utoipa::path(
    path = "/api/v1/budget-configs",
    get,
    tag = "budget-configs",
    operation_id = "budget-configs_find_all",
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
    // TODO: implement create find_all filter by limit and repetition_type

    User::has_any_role(roles, vec![Roles::User])?;

    let found_budget_configs = budget_configs_service::find_all(&state.db, &found_user).await?;
    let response: Vec<GetBudgetConfigResponse> = found_budget_configs
        .into_iter()
        .map(|budget_config| GetBudgetConfigResponse {
            id: budget_config.id,
            name: budget_config.name,
            limit: budget_config.limit.to_string(),
            description: budget_config.description,
            repetition_type: budget_config.repetition_type.to_value().to_string(),
        })
        .collect();

    Ok((
        StatusCode::OK,
        SuccessResponse::new("Successfully fetch budget configs", response),
    ))
}

#[utoipa::path(
    path = "/api/v1/budget-configs/{id}",
    get,
    tag = "budget-configs",
    operation_id = "budget-configs_get_by_id",
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
                limit: found_budget_config.limit.to_string(),
                description: found_budget_config.description,
                repetition_type: found_budget_config.repetition_type.to_value().to_string(),
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/budget-configs/{id}",
    put,
    tag = "budget-configs",
    operation_id = "budget-configs_update_by_id",
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
                limit: updated_budget_config.limit.to_string(),
                description: updated_budget_config.description,
                repetition_type: updated_budget_config.repetition_type.to_value().to_string(),
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/budget-configs/{id}",
    delete,
    tag = "budget-configs",
    operation_id = "budget-configs_delete_by_id",
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
