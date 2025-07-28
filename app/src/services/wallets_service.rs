use crate::dto::request::wallets_dto::{CreateWalletRequest, UpdateWalletRequest};
use crate::entities::{users, wallets};
use crate::errors::AppError;
use crate::repositories::wallets_repository;
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel};
use time::OffsetDateTime;

pub async fn create(
    db: &DatabaseConnection,
    user: &users::Model,
    payload: CreateWalletRequest,
) -> Result<wallets::Model, AppError> {
    let new_wallet = wallets::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        name: ActiveValue::Set(payload.name),
        description: ActiveValue::Set(payload.description),
        ..Default::default()
    };
    let new_wallet = wallets_repository::save(db, new_wallet).await?;
    Ok(new_wallet)
}

pub async fn find_all(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<Vec<wallets::Model>, AppError> {
    let found_wallets =
        wallets_repository::find_all_active_by_user_id_order_by_name_asc(db, user.id).await?;
    Ok(found_wallets)
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    wallet_id: i32,
) -> Result<wallets::Model, AppError> {
    let Some(found_wallet) =
        wallets_repository::find_active_by_id_and_user_id(db, wallet_id, user.id).await?
    else {
        return Err(AppError::NotFound(String::from("Wallet not found")));
    };
    Ok(found_wallet)
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    wallet_id: i32,
    payload: UpdateWalletRequest,
) -> Result<wallets::Model, AppError> {
    let mut found_wallet = get_by_id(db, user, wallet_id).await?.into_active_model();
    found_wallet.name = ActiveValue::Set(payload.name);
    found_wallet.description = ActiveValue::Set(payload.description);
    found_wallet.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());

    let updated_wallet = wallets_repository::save(db, found_wallet).await?;
    Ok(updated_wallet)
}

pub async fn delete_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    wallet_id: i32,
) -> Result<(), AppError> {
    let mut found_wallet = get_by_id(db, user, wallet_id).await?.into_active_model();
    found_wallet.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));
    wallets_repository::save(db, found_wallet).await?;
    Ok(())
}
