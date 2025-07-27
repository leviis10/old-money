use crate::dto::request::budget_configs_dto::create_budget_config_request::CreateBudgetConfigRequest;
use crate::dto::request::budget_configs_dto::update_budget_config_request::UpdateBudgetConfigRequest;
use crate::entities::{budget_configs, users};
use crate::errors::AppError;
use crate::repositories::budget_configs_repository;
use rust_decimal::Decimal;
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel};
use std::str::FromStr;
use time::OffsetDateTime;

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
    let new_budget_config = budget_configs_repository::save(db, new_budget_config).await?;

    // TODO: create initial budget

    Ok(new_budget_config)
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

pub async fn update_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    budget_config_id: i32,
    payload: UpdateBudgetConfigRequest,
) -> Result<budget_configs::Model, AppError> {
    let mut found_budget_config = get_by_id(db, user, budget_config_id)
        .await?
        .into_active_model();
    found_budget_config.name = ActiveValue::Set(payload.name);
    found_budget_config.limit = ActiveValue::Set(Decimal::from_str(&payload.limit)?);
    found_budget_config.description = ActiveValue::Set(payload.description);
    found_budget_config.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());

    let updated_budget_config = budget_configs_repository::update(db, found_budget_config).await?;
    Ok(updated_budget_config)
}

pub async fn delete_by_id(
    db: &DatabaseConnection,
    user: &users::Model,
    budget_config_id: i32,
) -> Result<(), AppError> {
    let mut found_budget_config = get_by_id(db, user, budget_config_id)
        .await?
        .into_active_model();
    found_budget_config.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));

    budget_configs_repository::update(db, found_budget_config).await?;

    Ok(())
}

pub async fn find_all(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<Vec<budget_configs::Model>, AppError> {
    let found_budget_configs =
        budget_configs_repository::find_all_active_by_user_id_order_by_name_asc(db, user.id)
            .await?;
    Ok(found_budget_configs)
}
