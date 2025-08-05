use crate::AppState;
use crate::dto::request::budgets_dto::{CreateBudgetRequest, UpdateBudgetRequest};
use crate::dto::response::budgets_dto::{
    CreateBudgetResponse, GetBudgetResponse, UpdateBudgetResponse,
};
use crate::dto::response::global::success_response::SuccessResponse;
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::budgets_service;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

#[utoipa::path(
    path = "/api/v1/budgets",
    post,
    tag = "budgets",
    operation_id = "budgets_create",
    request_body(
       content = CreateBudgetRequest,
       content_type = "application/json"
    ),
    responses(
        (status = 201, body = SuccessResponse<CreateBudgetResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<CreateBudgetRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateBudgetResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let new_budget = budgets_service::create(&state.db, &found_user, request).await?;

    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(
            "Successfully create new budget",
            CreateBudgetResponse {
                name: new_budget.name,
                start_date: new_budget.start_date,
                end_date: new_budget.end_date,
                current_amount: new_budget.current_amount.to_string(),
                limit: new_budget.limit.to_string(),
                description: new_budget.description,
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/budgets",
    get,
    tag = "budgets",
    operation_id = "budgets_find_all",
    responses(
        (status = 200, body = SuccessResponse<Vec<GetBudgetResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn find_all(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
) -> Result<(StatusCode, SuccessResponse<Vec<GetBudgetResponse>>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    // TODO: add filter by is_repeatable budget
    // TODO: add filter by name
    // TODO: add filter by date from
    // TODO: add filter by date to
    // TODO: add filter by limit
    // TODO: add filter by current_amount percentage
    // TODO: add sort by name asc and desc
    // TODO: add sort by current_amount asc and desc
    // TODO: add sort by current_amount percentage asc and desc
    // TODO: add sort by start_date asc and desc
    // TODO: add sort by end_date asc and desc
    // TODO: add sort by start and end date range asc and desc
    // TODO: add sort by limit asc and desc

    let found_budgets = budgets_service::find_all(&state.db, &found_user).await?;

    let response = found_budgets
        .into_iter()
        .map(GetBudgetResponse::from)
        .collect();

    Ok((
        StatusCode::OK,
        SuccessResponse::new("Successfully find all budgets", response),
    ))
}

#[utoipa::path(
    path = "/api/v1/budgets/{id}",
    get,
    tag = "budgets",
    operation_id = "budgets_get_by_id",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = SuccessResponse<GetBudgetResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Path(id): Path<i32>,
) -> Result<(StatusCode, SuccessResponse<GetBudgetResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let found_budget = budgets_service::get_by_id(&state.db, &found_user, id).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully get a budget",
            GetBudgetResponse::from(found_budget),
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/budgets/{id}",
    put,
    tag = "budgets",
    operation_id = "budgets_update_by_id",
    params(
        ("id" = i32, Path)
    ),
    request_body(
        content = UpdateBudgetRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 200, body = SuccessResponse<UpdateBudgetResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_by_id(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Path(id): Path<i32>,
    ValidatedJson(request): ValidatedJson<UpdateBudgetRequest>,
) -> Result<(StatusCode, SuccessResponse<UpdateBudgetResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let updated_budget = budgets_service::update_by_id(&state.db, &found_user, id, request).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully update a budget",
            UpdateBudgetResponse::from(updated_budget),
        ),
    ))
}

pub async fn delete_by_id(Path(id): Path<i32>) -> String {
    format!("Hello from DELETE /budgets/{id}")
}
