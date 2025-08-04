use crate::AppState;
use crate::dto::request::budgets_dto::CreateBudgetRequest;
use crate::dto::response::budgets_dto::CreateBudgetResponse;
use crate::dto::response::global::success_response::SuccessResponse;
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::budgets_service;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

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

pub async fn get_all() -> String {
    String::from("Hello from GET /budgets")
}

pub async fn get_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from GET /budgets/{id}")
}

pub async fn update_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from PUT /budgets/{id}")
}

pub async fn delete_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from DELETE /budgets/{id}")
}
