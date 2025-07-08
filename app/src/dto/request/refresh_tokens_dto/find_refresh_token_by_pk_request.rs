use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FindRefreshTokenByPkRequest {
    pub jti: Uuid,
    pub hashed_token: String,
    pub user_id: i32,
    pub expires_at: OffsetDateTime,
}
