use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct GetAllCategoriesParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
