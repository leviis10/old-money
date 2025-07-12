use crate::dto::request::user_roles_dto::insert_user_role_manual_request::InsertUserRoleManualRequest;
use crate::entities::user_roles;
use crate::errors::AppError;
use crate::repositories::user_roles_repository;
use sea_orm::{ActiveValue, DatabaseTransaction};

pub async fn create_manual(
    txn: &DatabaseTransaction,
    request: InsertUserRoleManualRequest,
) -> Result<user_roles::Model, AppError> {
    let new_user_role = user_roles::ActiveModel {
        user_id: ActiveValue::Set(request.user_id),
        role_id: ActiveValue::Set(request.role_id),
    };

    user_roles_repository::create_manual(txn, new_user_role).await
}
