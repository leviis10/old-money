use serde::Deserialize;
use time::Date;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequest {
    #[validate(range(min = 1, message = "category_id cannot be less than 1"))]
    pub category_id: i32,

    pub budget_id: Option<i32>,

    #[validate(range(min = 1, message = "category_id cannot be less than 1"))]
    pub wallet_id: i32,

    // TODO: add validation
    pub amount: String,

    pub description: Option<String>,

    // TODO: add validation
    pub flow_direction: String,

    pub issued_at: Date,
}
