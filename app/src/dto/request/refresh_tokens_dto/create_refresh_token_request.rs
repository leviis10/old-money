use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRefreshTokenRequest {
    pub jti: Uuid,
    pub refresh_token: String,
    pub expires_at: OffsetDateTime,
    pub user_id: i32,
}
