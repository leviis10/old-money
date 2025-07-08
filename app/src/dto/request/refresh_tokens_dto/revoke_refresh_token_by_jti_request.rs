use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RevokeRefreshTokenByJtiRequest {
    pub jti: Uuid,
}
