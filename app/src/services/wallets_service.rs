use crate::dto::request::wallets_dto::{
    CreateWalletRequest, FindAllWalletsParams, UpdateWalletRequest,
};
use crate::entities::sea_orm_active_enums::TransactionType;
use crate::entities::{transactions, users, wallets};
use crate::errors::AppError;
use crate::repositories::wallets_repository;
use sea_orm::{ActiveValue, ConnectionTrait, DatabaseConnection, IntoActiveModel};
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
    params: FindAllWalletsParams,
) -> Result<Vec<wallets::Model>, AppError> {
    let found_wallets =
        wallets_repository::find_all_active_by_user_id_order_by_name_asc(db, user.id, params)
            .await?;
    Ok(found_wallets)
}

pub async fn get_by_id(
    connection: &impl ConnectionTrait,
    user: &users::Model,
    wallet_id: i32,
) -> Result<wallets::Model, AppError> {
    let Some(found_wallet) =
        wallets_repository::find_active_by_id_and_user_id(connection, wallet_id, user.id).await?
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

pub async fn update_balance_after_transaction(
    connection: &impl ConnectionTrait,
    wallet: wallets::Model,
    transaction: &transactions::Model,
) -> Result<wallets::Model, AppError> {
    let wallet_balance = wallet.balance;

    let mut wallet = wallet.into_active_model();
    wallet.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());

    match transaction.flow_direction {
        TransactionType::Income => {
            wallet.balance = ActiveValue::Set(wallet_balance + transaction.amount);
        }
        TransactionType::Outcome => {
            wallet.balance = ActiveValue::Set(wallet_balance - transaction.amount);
        }
    }

    let updated_wallet = wallets_repository::save(connection, wallet).await?;
    Ok(updated_wallet)
}

pub async fn revert_transaction(
    connection: &impl ConnectionTrait,
    user: &users::Model,
    transaction: &transactions::Model,
) -> Result<wallets::Model, AppError> {
    let found_wallet = get_by_id(connection, user, transaction.wallet_id).await?;
    let wallet_balance = found_wallet.balance;

    let mut found_wallet = found_wallet.into_active_model();
    found_wallet.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());
    match transaction.flow_direction {
        TransactionType::Income => {
            found_wallet.balance = ActiveValue::Set(wallet_balance - transaction.amount);
        }
        TransactionType::Outcome => {
            found_wallet.balance = ActiveValue::Set(wallet_balance + transaction.amount);
        }
    }

    let updated_wallet = wallets_repository::save(connection, found_wallet).await?;
    Ok(updated_wallet)
}
