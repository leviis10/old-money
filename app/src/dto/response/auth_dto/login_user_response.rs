use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserResponse {
    pub access_token: String,
    pub refresh_token: String,
}
