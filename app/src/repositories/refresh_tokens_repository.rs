use crate::entities::prelude::RefreshTokens;
use crate::entities::refresh_tokens;
use crate::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    new_refresh_token: refresh_tokens::ActiveModel,
) -> Result<refresh_tokens::Model, AppError> {
    let refresh_token_model = new_refresh_token.insert(db).await?;
    Ok(refresh_token_model)
}

pub async fn revoke_using_model(
    db: &DatabaseConnection,
    refresh_token_model: refresh_tokens::ActiveModel,
) -> Result<(), AppError> {
    refresh_token_model.update(db).await?;
    Ok(())
}

pub async fn find_by_pk_and_hashed_token_and_user_id_and_expires_at_greater_than_and_deleted_at_is_null(
    db: &DatabaseConnection,
    jti: Uuid,
    hashed_token: &str,
    user_id: i32,
    expires_at: OffsetDateTime,
) -> Result<Option<refresh_tokens::Model>, AppError> {
    let refresh_token_model_option = RefreshTokens::find_by_id(jti)
        .filter(refresh_tokens::Column::HashedToken.eq(hashed_token))
        .filter(refresh_tokens::Column::ExpiresAt.gt(expires_at))
        .filter(refresh_tokens::Column::UserId.eq(user_id))
        .filter(refresh_tokens::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    Ok(refresh_token_model_option)
}
