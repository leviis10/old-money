use crate::entities::transactions;
use sea_orm::ActiveEnum;
use serde::Serialize;
use time::Date;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionResponse {
    pub id: i32,

    pub category_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_id: Option<i32>,

    pub wallet_id: i32,

    pub amount: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub flow_direction: String,

    pub issued_at: Date,
}

impl From<transactions::Model> for CreateTransactionResponse {
    fn from(transaction: transactions::Model) -> Self {
        CreateTransactionResponse {
            id: transaction.id,
            category_id: transaction.category_id,
            budget_id: transaction.budget_id,
            wallet_id: transaction.wallet_id,
            amount: transaction.amount.to_string(),
            description: transaction.description,
            flow_direction: transaction.flow_direction.into_value(),
            issued_at: transaction.issued_at,
        }
    }
}
