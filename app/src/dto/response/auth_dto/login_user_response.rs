use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserResponse {
    pub access_token: String,
    pub refresh_token: String,
}
