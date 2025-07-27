use crate::dto::request::budget_configs_dto::create_budget_config_request::CreateBudgetConfigRequest;
use crate::entities::{budget_configs, users};
use crate::errors::AppError;
use crate::repositories::budget_configs_repository;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, TryIntoModel};
use std::str::FromStr;

pub async fn create(
    db: &DatabaseConnection,
    user: &users::Model,
    payload: CreateBudgetConfigRequest,
) -> Result<budget_configs::Model, AppError> {
    let new_budget_config = budget_configs::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        name: ActiveValue::Set(payload.name),
        duration: ActiveValue::Set(payload.duration),
        limit: ActiveValue::Set(Decimal::from_str(&payload.limit)?),
        description: ActiveValue::Set(payload.description),
        ..Default::default()
    };
    let new_budget_config = new_budget_config.save(db).await?;

    // TODO: create initial budget

    Ok(new_budget_config.try_into_model()?)
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    budget_config_id: i32,
) -> Result<budget_configs::Model, AppError> {
    let found_budget_config =
        budget_configs_repository::get_active_by_id_and_user(db, budget_config_id, user).await?;

    let Some(found_budget_config) = found_budget_config else {
        return Err(AppError::NotFound(String::from("Budget Config not found")));
    };

    Ok(found_budget_config)
}
