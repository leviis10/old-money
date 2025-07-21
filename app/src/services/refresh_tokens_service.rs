use crate::dto::request::refresh_tokens_dto::create_refresh_token_request::CreateRefreshTokenRequest;
use crate::dto::request::refresh_tokens_dto::find_refresh_token_by_pk_request::FindRefreshTokenByPkRequest;
use crate::dto::request::refresh_tokens_dto::revoke_refresh_token_by_jti_request::RevokeRefreshTokenByJtiRequest;
use crate::entities::refresh_tokens;
use crate::errors::AppError;
use crate::repositories::refresh_tokens_repository;
use crate::utils::jwt_utils::RefreshTokenClaims;
use sea_orm::{ActiveValue, DatabaseConnection};
use time::OffsetDateTime;

pub async fn create(
    db: &DatabaseConnection,
    request: CreateRefreshTokenRequest,
) -> Result<refresh_tokens::Model, AppError> {
    let hashed_token = RefreshTokenClaims::hash(request.refresh_token.as_bytes());

    let new_refresh_token = refresh_tokens::ActiveModel {
        jti: ActiveValue::Set(request.jti),
        hashed_token: ActiveValue::Set(hashed_token),
        expires_at: ActiveValue::Set(request.expires_at),
        user_id: ActiveValue::Set(request.user_id),
        ..Default::default()
    };

    let new_refresh_token_model = refresh_tokens_repository::create(db, new_refresh_token).await?;
    Ok(new_refresh_token_model)
}

pub async fn revoke_by_jti(
    db: &DatabaseConnection,
    request: RevokeRefreshTokenByJtiRequest,
) -> Result<(), AppError> {
    let updated_refresh_token = refresh_tokens::ActiveModel {
        jti: ActiveValue::Unchanged(request.jti),
        deleted_at: ActiveValue::Set(Some(OffsetDateTime::now_utc())),
        ..Default::default()
    };
    refresh_tokens_repository::revoke_using_model(db, updated_refresh_token).await?;
    Ok(())
}

pub async fn find_by_pk(
    db: &DatabaseConnection,
    request: FindRefreshTokenByPkRequest,
) -> Result<refresh_tokens::Model, AppError> {
    let found_refresh_token_model = refresh_tokens_repository::find_by_pk_and_hashed_token_and_user_id_and_expires_at_greater_than_and_deleted_at_is_null(db, request.jti, &request.hashed_token, request.user_id, request.expires_at).await?;
    match found_refresh_token_model {
        Some(found_refresh_token) => Ok(found_refresh_token),
        None => Err(AppError::NotFound(String::from("Refresh Token Not Found"))),
    }
}
