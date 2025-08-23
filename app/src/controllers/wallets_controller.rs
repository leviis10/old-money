use crate::AppState;
use crate::dto::request::wallets_dto::{
    CreateWalletRequest, FindAllWalletsParams, UpdateWalletRequest,
};
use crate::dto::response::global::success_response::SuccessResponse;
use crate::dto::response::wallets_dto::{
    CreateWalletResponse, GetWalletResponse, UpdateWalletResponse,
};
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::wallets_service;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use std::sync::Arc;

#[utoipa::path(
    path = "/api/v1/wallets",
    post,
    tag = "wallets",
    operation_id = "wallets_create",
    request_body(
        content = CreateWalletRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, body = SuccessResponse<CreateWalletResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(request): ValidatedJson<CreateWalletRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateWalletResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let new_wallet = wallets_service::create(&state.db, &found_user, request).await?;

    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(
            "Successfully create new wallet",
            CreateWalletResponse {
                id: new_wallet.id,
                name: new_wallet.name,
                description: new_wallet.description,
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/wallets",
    get,
    tag = "wallets",
    operation_id = "wallets_find_all",
    params(
        ("max_fetch" = Option<u64>, Query),
    ),
    responses(
        (status = 200, body = SuccessResponse<Vec<GetWalletResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn find_all(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Query(params): Query<FindAllWalletsParams>,
) -> Result<(StatusCode, SuccessResponse<Vec<GetWalletResponse>>), AppError> {
    // TODO: implement filter by name, minimal amount, and maximal amount

    User::has_any_role(roles, vec![Roles::User])?;

    let found_wallets = wallets_service::find_all(&state.db, &found_user, params).await?;
    let response = found_wallets
        .into_iter()
        .map(|wallet| GetWalletResponse {
            id: wallet.id,
            name: wallet.name,
            balance: wallet.balance.to_string(),
            description: wallet.description,
        })
        .collect();

    Ok((
        StatusCode::OK,
        SuccessResponse::new("Successfully find all wallets", response),
    ))
}

#[utoipa::path(
    path = "/api/v1/wallets/{id}",
    get,
    tag = "wallets",
    operation_id = "wallets_get_by_id",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = SuccessResponse<GetWalletResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Path(id): Path<i32>,
) -> Result<(StatusCode, SuccessResponse<GetWalletResponse>), AppError> {
    // TODO: Show transactions for this month

    User::has_any_role(roles, vec![Roles::User])?;

    let found_wallet = wallets_service::get_by_id(&state.db, &found_user, id).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully get a wallet",
            GetWalletResponse {
                id: found_wallet.id,
                name: found_wallet.name,
                balance: found_wallet.balance.to_string(),
                description: found_wallet.description,
            },
        ),
    ))
}

#[utoipa::path(
    path = "/api/v1/wallets/{id}",
    put,
    tag = "wallets",
    operation_id = "wallets_update_by_id",
    params(
        ("id" = i32, Path)
    ),
    request_body(
        content = UpdateWalletRequest,
        content_type = "application/json",
    ),
    responses(
        (status = 200, body = SuccessResponse<UpdateWalletResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_by_id(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Path(id): Path<i32>,
    ValidatedJson(request): ValidatedJson<UpdateWalletRequest>,
) -> Result<(StatusCode, SuccessResponse<UpdateWalletResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;

    let updated_wallet = wallets_service::update_by_id(&state.db, &found_user, id, request).await?;

    Ok((
        StatusCode::OK,
        SuccessResponse::new(
            "Successfully update a wallet",
            UpdateWalletResponse {
                id: updated_wallet.id,
                name: updated_wallet.name,
                balance: updated_wallet.balance.to_string(),
                description: updated_wallet.description,
            },
        ),
    ))
}

// TODO: Create adjust balance feature

#[utoipa::path(
    path = "/api/v1/wallets/{id}",
    delete,
    tag = "wallets",
    operation_id = "wallets_delete_by_id",
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

    wallets_service::delete_by_id(&state.db, &found_user, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
