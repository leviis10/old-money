use serde::Deserialize;
use time::Date;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBudgetRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,

    pub start_date: Option<Date>,

    pub end_date: Option<Date>,

    // TODO: add custom validation for decimal data type
    pub limit: String,

    pub description: Option<String>,

    // TODO: add custom validation for repetition_type enum
    pub repetition_type: Option<String>,
}
