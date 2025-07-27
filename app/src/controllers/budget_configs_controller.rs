use crate::AppState;
use crate::constants::time_constants::DATE_FORMATTER;
use crate::dto::request::budget_configs_dto::create_budget_config_request::CreateBudgetConfigRequest;
use crate::dto::response::budget_configs_dto::create_budget_config_response::CreateBudgetConfigResponse;
use crate::dto::response::budget_configs_dto::get_budget_config_by_id_response::GetBudgetConfigByIdResponse;
use crate::dto::response::global::success_response::SuccessResponse;
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::budget_configs_service;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

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

pub async fn get_all(User(_found_user, roles): User) -> Result<String, AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    Ok(String::from("Hello from GET /budget-config"))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    User(found_user, roles): User,
) -> Result<(StatusCode, SuccessResponse<GetBudgetConfigByIdResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let found_budget_config = budget_configs_service::get_by_id(&state.db, &found_user, id).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully get a budget config",
            GetBudgetConfigByIdResponse {
                id: found_budget_config.id,
                name: found_budget_config.name,
                duration: found_budget_config.duration,
                limit: found_budget_config.limit.to_string(),
                description: found_budget_config.description,
            },
        ),
    ))
}

pub async fn update_by_id(
    Path(id): Path<i32>,
    User(_found_user, roles): User,
) -> Result<String, AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    Ok(format!("Hello from PUT /budget-config/{id}"))
}

pub async fn delete_by_id(
    Path(id): Path<i32>,
    User(_found_user, roles): User,
) -> Result<String, AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    Ok(format!("Hello from DELETE /budget-config/{id}"))
}
