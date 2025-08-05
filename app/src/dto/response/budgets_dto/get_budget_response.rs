use crate::entities::budgets;
use serde::Serialize;
use time::Date;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBudgetResponse {
    pub id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_config_id: Option<i32>,

    pub name: String,

    pub start_date: Date,

    pub end_date: Date,

    pub current_amount: String,

    pub limit: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<budgets::Model> for GetBudgetResponse {
    fn from(budget: budgets::Model) -> Self {
        GetBudgetResponse {
            id: budget.id,
            budget_config_id: budget.budget_config_id,
            name: budget.name,
            start_date: budget.start_date,
            end_date: budget.end_date,
            current_amount: budget.current_amount.to_string(),
            limit: budget.limit.to_string(),
            description: budget.description,
        }
    }
}
