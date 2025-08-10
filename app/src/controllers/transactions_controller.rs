use crate::AppState;
use crate::dto::request::transactions_dto::{CreateTransactionRequest, UpdateTransactionRequest};
use crate::dto::response::global::success_response::SuccessResponse;
use crate::dto::response::transactions_dto::{
    CreateTransactionResponse, GetTransactionResponse, UpdateTransactionResponse,
};
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::transactions_service;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

#[utoipa::path(
    path = "/api/v1/transactions",
    post,
    tag = "transactions",
    operation_id = "transactions_create",
    request_body(
        content = CreateTransactionRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, body = SuccessResponse<CreateTransactionResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<CreateTransactionRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateTransactionResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let new_transaction = transactions_service::create(&state.db, &found_user, request).await?;

    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(
            "Successfully create a transaction",
            CreateTransactionResponse::from(new_transaction),
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/transactions",
    get,
    tag = "transactions",
    operation_id = "transactions_find_all",
    responses(
        (status = 200, body = SuccessResponse<Vec<GetTransactionResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn find_all(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
) -> Result<(StatusCode, SuccessResponse<Vec<GetTransactionResponse>>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    // TODO: add filter by date
    // TODO: add filter by category
    // TODO: add filter by amount
    // TODO: add filter by flow_direction
    // TODO: add filter by wallet

    let found_transactions = transactions_service::find_all(&state.db, &found_user).await?;
    let response = found_transactions
        .into_iter()
        .map(GetTransactionResponse::from)
        .collect();

    Ok((
        StatusCode::OK,
        SuccessResponse::new("Successfully found all transactions", response),
    ))
}

#[utoipa::path(
    path = "/api/v1/transactions/{id}",
    get,
    tag = "transactions",
    operation_id = "transactions_get_by_id",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = SuccessResponse<GetTransactionResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Path(id): Path<i32>,
) -> Result<(StatusCode, SuccessResponse<GetTransactionResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    let found_transaction = transactions_service::get_by_id(&state.db, &found_user, id).await?;
    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully found a transaction",
            GetTransactionResponse::from(found_transaction),
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/transactions/{id}",
    put,
    tag = "transactions",
    operation_id = "transactions_update_by_id",
    params(
        ("id" = i32, Path)
    ),
    request_body(
        content = UpdateTransactionRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 200, body = SuccessResponse<UpdateTransactionResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_by_id(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Path(id): Path<i32>,
    ValidatedJson(request): ValidatedJson<UpdateTransactionRequest>,
) -> Result<(StatusCode, SuccessResponse<UpdateTransactionResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    let updated_transaction =
        transactions_service::update_by_id(&state.db, &found_user, id, request).await?;
    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully updated a transaction",
            UpdateTransactionResponse::from(updated_transaction),
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/transactions/{id}",
    delete,
    tag = "transactions",
    operation_id = "transactions_delete_by_id",
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
    User(found_user, roles): User,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    transactions_service::delete_by_id(&state.db, &found_user, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
