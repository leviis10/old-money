use crate::entities::prelude::BudgetConfigs;
use crate::entities::{budget_configs, users};
use crate::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_active_by_id_and_user(
    db: &DatabaseConnection,
    budget_config_id: i32,
    user: &users::Model,
) -> Result<Option<budget_configs::Model>, AppError> {
    let found_budget_config = BudgetConfigs::find_by_id(budget_config_id)
        .filter(budget_configs::Column::UserId.eq(user.id))
        .filter(budget_configs::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    Ok(found_budget_config)
}
