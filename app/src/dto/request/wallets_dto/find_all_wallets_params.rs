use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct FindAllWalletsParams {
    pub max_fetch: Option<u64>,
}
