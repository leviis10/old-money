use crate::dto::request::budget_configs_dto::create_budget_config_request::CreateBudgetConfigRequest;
use crate::dto::request::budgets_dto::CreateBudgetRequest;
use crate::entities::sea_orm_active_enums::RepetitionTypeEnum;
use crate::entities::{budgets, users};
use crate::errors::AppError;
use crate::repositories::budgets_repository;
use crate::services::budget_configs_service;
use rust_decimal::Decimal;
use sea_orm::{ActiveValue, DatabaseConnection};
use std::str::FromStr;
use time::{Date, Duration, Month, OffsetDateTime};

pub async fn create(
    db: &DatabaseConnection,
    user: &users::Model,
    payload: CreateBudgetRequest,
) -> Result<budgets::Model, AppError> {
    match payload.repetition_type {
        Some(ref repetition_type) => {
            let new_budget_config = budget_configs_service::create(
                db,
                user,
                CreateBudgetConfigRequest {
                    name: String::from(&payload.name),
                    limit: String::from(&payload.limit),
                    description: payload.description.to_owned(),
                    repetition_type: String::from(repetition_type),
                },
            )
            .await?;

            // define start_date and end_date
            let current_time = OffsetDateTime::now_utc();
            let start_date;
            let end_date;
            match new_budget_config.repetition_type {
                RepetitionTypeEnum::Daily => {
                    let current_date = current_time.date();
                    start_date = current_date;
                    end_date = current_date;
                }
                RepetitionTypeEnum::Weekly => {
                    let current_date = current_time.date();
                    start_date = current_date
                        - Duration::days(current_date.weekday().number_days_from_monday().into());
                    end_date = start_date + Duration::days(7);
                }
                RepetitionTypeEnum::Monthly => {
                    let current_year = current_time.year();
                    let current_month = current_time.month();
                    start_date = Date::from_calendar_date(current_year, current_month, 1)?;
                    end_date = Date::from_calendar_date(
                        current_year,
                        current_month,
                        current_month.length(current_year),
                    )?;
                }
                RepetitionTypeEnum::Yearly => {
                    let current_year = current_time.year();
                    start_date = Date::from_calendar_date(current_year, Month::January, 1)?;
                    end_date = Date::from_calendar_date(current_year, Month::December, 31)?;
                }
            };

            let new_budget = create_budget(
                db,
                user,
                payload,
                start_date,
                end_date,
                Some(new_budget_config.id),
            )
            .await?;
            Ok(new_budget)
        }
        None => {
            let (Some(start_date), Some(end_date)) = (payload.start_date, payload.end_date) else {
                return Err(AppError::ParseBody(String::from(
                    "invalid start_date or end_date",
                )));
            };
            let new_budget = create_budget(db, user, payload, start_date, end_date, None).await?;
            Ok(new_budget)
        }
    }
}

async fn create_budget(
    db: &DatabaseConnection,
    user: &users::Model,
    payload: CreateBudgetRequest,
    start_date: Date,
    end_date: Date,
    budget_config_id: Option<i32>,
) -> Result<budgets::Model, AppError> {
    let limit = Decimal::from_str(&payload.limit)?;

    let new_budget = budgets::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        budget_config_id: ActiveValue::Set(budget_config_id),
        name: ActiveValue::Set(payload.name),
        start_date: ActiveValue::Set(start_date),
        end_date: ActiveValue::Set(end_date),
        limit: ActiveValue::Set(limit),
        description: ActiveValue::Set(payload.description),
        ..Default::default()
    };
    let new_budget = budgets_repository::save(db, new_budget).await?;
    if let Some(budget_config_id) = budget_config_id {
        budget_configs_service::update_last_create(db, user, budget_config_id).await?;
    }

    Ok(new_budget)
}
