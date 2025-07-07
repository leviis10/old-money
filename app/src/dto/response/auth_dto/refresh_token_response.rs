use derive_builder::Builder;
use serde::Serialize;

#[derive(Serialize, Builder)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResponse {
    access_token: String,
    refresh_token: String,
}