use crate::entities::refresh_tokens;
use crate::repositories::refresh_tokens_repository;
use sea_orm::{ActiveValue, DatabaseConnection};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    jti: Uuid,
    hashed_token: String,
    expires_at: OffsetDateTime,
    user_id: i32,
) -> refresh_tokens::Model {
    let new_refresh_token = refresh_tokens::ActiveModel {
        jti: ActiveValue::Set(jti),
        hashed_token: ActiveValue::Set(hashed_token),
        expires_at: ActiveValue::Set(expires_at),
        user_id: ActiveValue::Set(user_id),
        ..Default::default()
    };

    refresh_tokens_repository::create(db, new_refresh_token).await
}

pub async fn revoke_using_model(db: &DatabaseConnection, refresh_token_model: refresh_tokens::Model) {
    let updated_refresh_token = refresh_tokens::ActiveModel {
        jti: ActiveValue::Unchanged(refresh_token_model.jti),
        deleted_at: ActiveValue::Set(Some(OffsetDateTime::now_utc())),
        ..Default::default()
    };
    refresh_tokens_repository::revoke_using_model(db, updated_refresh_token).await;
}

pub async fn find_by_pk_and_hashed_token_and_user_id_and_expires_at_greater_than_and_deleted_at_is_null(db: &DatabaseConnection, jti: Uuid, hashed_token: &str, user_id: i32, expires_at: OffsetDateTime) -> Option<refresh_tokens::Model> {
    refresh_tokens_repository::find_by_pk_and_hashed_token_and_user_id_and_expires_at_greater_than_and_deleted_at_is_null(db, jti, hashed_token, user_id, expires_at).await
}
