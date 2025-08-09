use crate::entities::prelude::BudgetConfigs;
use crate::entities::{budget_configs, users};
use crate::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, TryIntoModel,
};

pub async fn get_active_by_id_and_user(
    connection: &impl ConnectionTrait,
    budget_config_id: i32,
    user: &users::Model,
) -> Result<Option<budget_configs::Model>, AppError> {
    let found_budget_config = BudgetConfigs::find_by_id(budget_config_id)
        .filter(budget_configs::Column::UserId.eq(user.id))
        .filter(budget_configs::Column::DeletedAt.is_null())
        .one(connection)
        .await?;

    Ok(found_budget_config)
}

pub async fn update(
    db: &DatabaseConnection,
    budget_config: budget_configs::ActiveModel,
) -> Result<budget_configs::Model, AppError> {
    let updated_budget_config = budget_config.update(db).await?;
    Ok(updated_budget_config)
}

pub async fn save(
    connection: &impl ConnectionTrait,
    budget_configs: budget_configs::ActiveModel,
) -> Result<budget_configs::Model, AppError> {
    let new_budget_config = budget_configs.save(connection).await?;
    Ok(new_budget_config.try_into_model()?)
}

pub async fn find_all_active_by_user_id_order_by_name_asc(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<budget_configs::Model>, AppError> {
    let found_budget_configs = BudgetConfigs::find()
        .filter(budget_configs::Column::UserId.eq(user_id))
        .filter(budget_configs::Column::DeletedAt.is_null())
        .order_by_asc(budget_configs::Column::Name)
        .all(db)
        .await?;

    Ok(found_budget_configs)
}
