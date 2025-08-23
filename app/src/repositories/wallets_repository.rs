use crate::dto::request::wallets_dto::FindAllWalletsParams;
use crate::entities::prelude::Wallets;
use crate::entities::wallets;
use crate::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, TryIntoModel,
};

pub async fn save(
    db: &impl ConnectionTrait,
    wallet: wallets::ActiveModel,
) -> Result<wallets::Model, AppError> {
    let new_wallet = wallet.save(db).await?.try_into_model()?;
    Ok(new_wallet)
}

pub async fn find_all_active_by_user_id_order_by_name_asc(
    db: &DatabaseConnection,
    user_id: i32,
    params: FindAllWalletsParams,
) -> Result<Vec<wallets::Model>, AppError> {
    let found_wallets_builder = Wallets::find()
        .filter(wallets::Column::UserId.eq(user_id))
        .filter(wallets::Column::DeletedAt.is_null())
        .order_by_asc(wallets::Column::Name);

    let Some(max_fetch) = params.max_fetch else {
        let found_wallets = found_wallets_builder.all(db).await?;
        return Ok(found_wallets);
    };

    let found_wallets = found_wallets_builder.limit(max_fetch).all(db).await?;

    Ok(found_wallets)
}

pub async fn find_active_by_id_and_user_id(
    db: &impl ConnectionTrait,
    wallet_id: i32,
    user_id: i32,
) -> Result<Option<wallets::Model>, AppError> {
    let found_wallet = Wallets::find_by_id(wallet_id)
        .filter(wallets::Column::UserId.eq(user_id))
        .filter(wallets::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    Ok(found_wallet)
}
