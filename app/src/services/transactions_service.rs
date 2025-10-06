use crate::dto::request::transactions_dto::{CreateTransactionRequest, UpdateTransactionRequest};
use crate::entities::sea_orm_active_enums::TransactionType;
use crate::entities::{transactions, users};
use crate::errors::AppError;
use crate::repositories::transactions_repository;
use crate::services::budgets_service;
use crate::services::categories_service;
use crate::services::wallets_service;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveEnum, ActiveValue, ConnectionTrait, DatabaseConnection, IntoActiveModel,
    TransactionTrait, TryIntoModel,
};
use std::str::FromStr;
use time::OffsetDateTime;

pub async fn create(
    db: &DatabaseConnection,
    user: &users::Model,
    payload: CreateTransactionRequest,
) -> Result<transactions::Model, AppError> {
    let txn = db.begin().await?;
    let found_wallet = wallets_service::get_by_id(&txn, user, payload.wallet_id).await?;

    let mut budget_id = None;
    let mut found_budget = None;
    if let Some(payload_budget_id) = payload.budget_id {
        found_budget = Some(budgets_service::get_by_id(&txn, user, payload_budget_id).await?);
        budget_id = Some(found_budget.as_ref().unwrap().id);
    }

    let found_category = categories_service::get_by_id(&txn, user, payload.category_id).await?;

    let new_transaction = transactions::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        category_id: ActiveValue::Set(found_category.id),
        budget_id: ActiveValue::Set(budget_id),
        wallet_id: ActiveValue::Set(found_wallet.id),
        amount: ActiveValue::Set(Decimal::from_str(&payload.amount)?),
        description: ActiveValue::Set(payload.description),
        flow_direction: ActiveValue::Set(TransactionType::try_from_value(&payload.flow_direction)?),
        issued_at: ActiveValue::Set(payload.issued_at),
        ..Default::default()
    };
    let new_transaction = transactions_repository::save(&txn, new_transaction).await?;

    wallets_service::update_balance_after_transaction(&txn, found_wallet, &new_transaction).await?;

    if let Some(found_budget) = found_budget
        && new_transaction.flow_direction == TransactionType::Outcome
    {
        budgets_service::update_amount_after_transaction(
            &txn,
            found_budget,
            new_transaction.amount,
        )
        .await?;
    }
    txn.commit().await?;

    Ok(new_transaction)
}

pub async fn find_all(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<Vec<transactions::Model>, AppError> {
    let found_transactions =
        transactions_repository::find_all_active_by_user_id_order_by_issued_at_and_created_at_desc(
            db, user.id,
        )
        .await?;
    Ok(found_transactions)
}

pub async fn get_by_id(
    connection: &impl ConnectionTrait,
    user: &users::Model,
    transaction_id: i32,
) -> Result<transactions::Model, AppError> {
    let found_transaction =
        transactions_repository::get_active_by_id_and_user_id(connection, transaction_id, user.id)
            .await?;
    let Some(found_transaction) = found_transaction else {
        return Err(AppError::NotFound(String::from("Transaction not found")));
    };
    Ok(found_transaction)
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    transaction_id: i32,
    payload: UpdateTransactionRequest,
) -> Result<transactions::Model, AppError> {
    let txn = db.begin().await?;

    let found_transaction = get_by_id(&txn, user, transaction_id).await?;
    revert_transaction(&txn, user, &found_transaction).await?;

    let updated_transaction = apply_transaction(&txn, user, found_transaction, payload).await?;

    txn.commit().await?;

    Ok(updated_transaction)
}

async fn revert_transaction(
    connection: &impl ConnectionTrait,
    user: &users::Model,
    transaction: &transactions::Model,
) -> Result<(), AppError> {
    wallets_service::revert_transaction(connection, user, transaction).await?;

    if let Some(budget_id) = transaction.budget_id
        && transaction.flow_direction == TransactionType::Outcome
    {
        budgets_service::revert_transaction(connection, user, budget_id, transaction.amount)
            .await?;
    }

    Ok(())
}

async fn apply_transaction(
    connection: &impl ConnectionTrait,
    user: &users::Model,
    transaction: transactions::Model,
    payload: UpdateTransactionRequest,
) -> Result<transactions::Model, AppError> {
    let transaction_category_id = transaction.category_id;

    let mut transaction = transaction.into_active_model();
    transaction.amount = ActiveValue::Set(Decimal::from_str(&payload.amount)?);
    transaction.description = ActiveValue::Set(payload.description);
    transaction.flow_direction =
        ActiveValue::Set(TransactionType::try_from_value(&payload.flow_direction)?);
    transaction.budget_id = ActiveValue::Set(payload.budget_id);
    transaction.issued_at = ActiveValue::Set(payload.issued_at);
    transaction.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());
    if transaction_category_id != payload.category_id {
        let found_category =
            categories_service::get_by_id(connection, user, payload.category_id).await?;
        transaction.category_id = ActiveValue::Set(found_category.id);
    }
    if let Some(payload_budget_id) = payload.budget_id {
        let found_budget = budgets_service::get_by_id(connection, user, payload_budget_id).await?;
        budgets_service::update_amount_after_transaction(
            connection,
            found_budget,
            Decimal::from_str(&payload.amount)?,
        )
        .await?;
    }

    let found_wallet = wallets_service::get_by_id(connection, user, payload.wallet_id).await?;
    transaction.wallet_id = ActiveValue::Set(found_wallet.id);
    wallets_service::update_balance_after_transaction(
        connection,
        found_wallet,
        &transaction.clone().try_into_model()?,
    )
    .await?;

    let updated_transaction = transactions_repository::save(connection, transaction).await?;
    Ok(updated_transaction)
}

pub async fn delete_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    transaction_id: i32,
) -> Result<(), AppError> {
    let txn = db.begin().await?;

    let found_transaction = get_by_id(&txn, user, transaction_id).await?;
    revert_transaction(&txn, user, &found_transaction).await?;

    let mut found_transaction = found_transaction.into_active_model();
    found_transaction.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));

    transactions_repository::save(&txn, found_transaction).await?;

    txn.commit().await?;

    Ok(())
}
