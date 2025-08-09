use crate::entities::prelude::Transactions;
use crate::entities::transactions;
use crate::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder,
    TryIntoModel,
};

pub async fn save(
    connection: &impl ConnectionTrait,
    transaction: transactions::ActiveModel,
) -> Result<transactions::Model, AppError> {
    let transaction = transaction.save(connection).await?.try_into_model()?;
    Ok(transaction)
}

pub async fn find_all_active_by_user_id_order_by_issued_at_and_created_at_desc(
    connection: &impl ConnectionTrait,
    user_id: i32,
) -> Result<Vec<transactions::Model>, AppError> {
    let found_transactions = Transactions::find()
        .filter(transactions::Column::DeletedAt.is_null())
        .filter(transactions::Column::UserId.eq(user_id))
        .order_by_desc(transactions::Column::IssuedAt)
        .order_by_desc(transactions::Column::CreatedAt)
        .all(connection)
        .await?;
    Ok(found_transactions)
}

pub async fn get_active_by_id_and_user_id(
    connection: &impl ConnectionTrait,
    transaction_id: i32,
    user_id: i32,
) -> Result<Option<transactions::Model>, AppError> {
    let found_transaction = Transactions::find_by_id(transaction_id)
        .filter(transactions::Column::UserId.eq(user_id))
        .filter(transactions::Column::DeletedAt.is_null())
        .one(connection)
        .await?;
    Ok(found_transaction)
}
