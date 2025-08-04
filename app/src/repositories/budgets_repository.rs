use crate::entities::budgets;
use crate::errors::AppError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

pub async fn save(
    db: &DatabaseConnection,
    budget: budgets::ActiveModel,
) -> Result<budgets::Model, AppError> {
    let budget = budget.save(db).await?.try_into_model()?;
    Ok(budget)
}
