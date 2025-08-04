use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub id: i32,

    pub username: String,

    pub email: String,
}
