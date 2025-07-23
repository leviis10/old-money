use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryResponse {
    pub id: i32,

    pub name: String,
}
