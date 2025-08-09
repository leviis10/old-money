use crate::entities::budgets;
use crate::entities::prelude::Budgets;
use crate::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, TryIntoModel,
};

pub async fn save(
    db: &impl ConnectionTrait,
    budget: budgets::ActiveModel,
) -> Result<budgets::Model, AppError> {
    let budget = budget.save(db).await?.try_into_model()?;
    Ok(budget)
}

pub async fn find_all_active_by_user_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<budgets::Model>, AppError> {
    let found_budgets = Budgets::find()
        .filter(budgets::Column::UserId.eq(user_id))
        .filter(budgets::Column::DeletedAt.is_null())
        .order_by_asc(budgets::Column::Name)
        .all(db)
        .await?;

    Ok(found_budgets)
}

pub async fn get_active_by_id_and_user_id(
    db: &impl ConnectionTrait,
    budget_id: i32,
    user_id: i32,
) -> Result<Option<budgets::Model>, AppError> {
    let found_budget = Budgets::find_by_id(budget_id)
        .filter(budgets::Column::UserId.eq(user_id))
        .filter(budgets::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    Ok(found_budget)
}
